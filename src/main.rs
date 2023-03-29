use image::GenericImageView;
use std::fmt::Display;
use std::path::Path;

#[derive(Debug)]
enum ImageOrientation {
    Landscape,
    Portrait,
    Square,
}

impl ImageOrientation {
    fn from(img: &image::DynamicImage) -> ImageOrientation {
        let (width, height) = img.dimensions();
        if width > height {
            ImageOrientation::Landscape
        } else if height > width {
            ImageOrientation::Portrait
        } else {
            ImageOrientation::Square
        }
    }
}

// ImageSizes enum where large is 1024-Infinite, medium is 640-1024, small is 320-640, xsmall is 240-320, thumbnail is 128-240, avatar is 0-128
#[derive(Debug)]
enum ImageSize {
    Large,
    Medium,
    Small,
    XSmall,
    Thumbnail,
    Avatar,
}

impl Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageSize::Large => write!(f, "lg"),
            ImageSize::Medium => write!(f, "md"),
            ImageSize::Small => write!(f, "sm"),
            ImageSize::XSmall => write!(f, "xs"),
            ImageSize::Thumbnail => write!(f, "thumb"),
            ImageSize::Avatar => write!(f, "avatar"),
        }
    }
}

impl ImageSize {
    fn values() -> &'static [ImageSize] {
        static VALUES: [ImageSize; 6] = [
            ImageSize::Large,
            ImageSize::Medium,
            ImageSize::Small,
            ImageSize::XSmall,
            ImageSize::Thumbnail,
            ImageSize::Avatar,
        ];
        &VALUES
    }

    fn max_dimension(self: &ImageSize) -> u32 {
        match self {
            ImageSize::Large => 1024,
            ImageSize::Medium => 640,
            ImageSize::Small => 320,
            ImageSize::XSmall => 240,
            ImageSize::Thumbnail => 128,
            ImageSize::Avatar => 64,
        }
    }

    fn from(img: &image::DynamicImage) -> ImageSize {
        let (width, height) = img.dimensions();
        let largest_dim = std::cmp::max(width, height);
        if largest_dim > 1024 {
            ImageSize::Large
        } else if largest_dim > 640 {
            ImageSize::Medium
        } else if largest_dim > 320 {
            ImageSize::Small
        } else if largest_dim > 240 {
            ImageSize::XSmall
        } else if largest_dim > 128 {
            ImageSize::Thumbnail
        } else {
            ImageSize::Avatar
        }
    }
}

// measure the time it takes to run a function
fn time_it<F>(f: F) -> std::time::Duration
where
    F: FnOnce(),
{
    let start = std::time::Instant::now();
    f();
    start.elapsed()
}

fn main() {
    let photo_dir = Path::new("/home/will/Pictures/inspections");
    let image_filenames = ["sliding_door_issue", "bathroom", "kitchen", "living"];

    for filename in image_filenames.iter() {
        let path = photo_dir.join(format!("{}.jpg", filename));
        let img = image::open(path).unwrap();
        let orientation = ImageOrientation::from(&img);
        let size = ImageSize::from(&img);
        println!("Image: {:?} {:?}", orientation, size);

        for size in ImageSize::values().iter() {
            let mut save_path: Option<String> = None;
            let dur = time_it(|| {
                println!("Image Size: {:?}", size);
                let resized_img = img.resize(
                    size.max_dimension(),
                    size.max_dimension(),
                    image::imageops::FilterType::Triangle,
                );

                let output_dir = photo_dir.join("output");
                std::fs::create_dir_all(&output_dir).unwrap();

                let new_path = output_dir.join(format!("{}.{}.jpg", filename, size));
                save_path = Some(new_path.to_str().unwrap().to_string());
                resized_img.save(new_path).unwrap();
            });
            println!("Resized image ({:?}) in {:?}", save_path.unwrap(), dur);
        }
    }
}
