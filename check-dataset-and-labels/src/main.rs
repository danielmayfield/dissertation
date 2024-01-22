use std::collections::HashMap;
use std::env;
use std::io;
use std::fs;
use std::io::Write;
use serde_json::Value;

// set global variables
// Change these paths to the location of the dataset and labels on your machine
const DATASET_PATH: &str = "F:/Dissertation/Berkeley DeepDrive Dataset/bdd100k_images_100k/bdd100k/images/100k";
const LABELS_PATH: &str = "F:/Dissertation/Berkeley DeepDrive Dataset/bdd100k_det_20_labels_trainval/bdd100k/labels/det_20";

// object classes to be detected
const OBJECT_CLASSES: [&str; 10] = [
    "pedestrian",
    "rider",
    "car",
    "truck",
    "bus",
    "train",
    "motorcycle",
    "bicycle",
    "traffic light",
    "traffic sign"
];

// image size - width and height
const IMAGE_SIZE: (f64, f64) = (1280.0, 720.0);

fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    println!("Checking Dataset and Label Names");

    // get image names
    let image_names = get_image_names();
    println!("Number of images: {}", image_names.len());

    // get label names
    let label_names = get_label_names();

    let matched_names = match_image_and_label_names(image_names, label_names);
    println!("Number of matched names: {}", matched_names.len());

    create_train_val_test_sets(matched_names);
}

fn get_image_names() -> Vec<String>{
    let mut image_names = Vec::new();

    match fs::read_dir(DATASET_PATH) {
        Ok(entries) => {
            for entry in entries {
                match fs::read_dir(entry.unwrap().path()) {
                    Ok(entries) => {
                        for entry in entries {
                            // add image name to array
                            image_names.push(entry.unwrap().path().file_name().unwrap().to_str().unwrap().to_string());
                        }
                    },
                    Err(e) => println!("Error reading directory: {}", e),
                }
            }
        },
        Err(e) => println!("Error reading directory: {}", e),
    }

    // return array
    return image_names;
}

fn get_label_names() -> Vec<String>{
    let mut label_names = Vec::new();

    // get label files
    match fs::read_dir(LABELS_PATH) {
        Ok(entries) => {
            for entry in entries {
                // open json file
                let file = fs::File::open(entry.unwrap().path()).unwrap();
                let reader = io::BufReader::new(file);

                // parse json file
                let json: Value = serde_json::from_reader(reader).unwrap();

                // loop through json file
                for object in json.as_array().unwrap() {
                    // get image name
                    let image_name = object["name"].as_str().unwrap().to_string();

                    // add image name to array
                    label_names.push(image_name);
                }

                println!("Number of labels: {}", label_names.len());
            }
        },
        Err(e) => println!("Error reading directory: {}", e),
    }

    // return array
    return label_names;
}

fn match_image_and_label_names(image_names: Vec<String>, label_names: Vec<String>) -> Vec<String>{
    let mut matched_names = Vec::new();

    // sort label names
    let mut label_names = label_names;
    label_names.sort();

    // loop through image names
    for image_name in image_names {
        // binary search for image name in label names
        let result = label_names.binary_search(&image_name);

        // if image name is found
        if result.is_ok() {
            // add image name to array
            matched_names.push(image_name);
        } else {
            // if image name is not found
            println!("Image name not found: {}", image_name);
        }
    }

    // return array
    return matched_names;
}

fn copy_image(new_dir:String, image_name: String, set: String){
    // try catch copy image from the three folders
    let train_result = fs::copy(DATASET_PATH.to_string() + "/train/" + &image_name, new_dir.clone() + &set + "/images/" + &image_name);
    let val_result = fs::copy(DATASET_PATH.to_string() + "/val/" + &image_name, new_dir.clone() + &set + "/images/" + &image_name);
    let test_result = fs::copy(DATASET_PATH.to_string() + "/test/" + &image_name, new_dir.clone() + &set +"/images/" + &image_name);

    match train_result {
        Ok(_) => {},
        Err(_e) => {
            match val_result {
                Ok(_) => {},
                Err(_e) => {
                    match test_result {
                        Ok(_) => {},
                        Err(e) => println!("Error copying image: {}", e),
                    }
                },
            }
        },
    }
}

fn create_folder_structure(new_dir: String) {
    let new_folder_names: [&str; 3] = ["train", "val", "test"];

    // create new folder
    fs::create_dir_all(new_dir.clone() + "train").unwrap();
    fs::create_dir_all(new_dir.clone() + "val").unwrap();
    fs::create_dir_all(new_dir.clone() + "test").unwrap();

    for folder_name in new_folder_names.iter() {
        // create new folder
        fs::create_dir_all(new_dir.clone() + folder_name).unwrap();

        // create images and labels folders in new folder
        fs::create_dir_all(new_dir.clone() + folder_name + "/images").unwrap();
        fs::create_dir_all(new_dir.clone() + folder_name + "/labels").unwrap();
    }

}

fn create_data_yaml(new_dir: String) {
    // create data yaml file
    let mut data_yaml_file = fs::File::create(new_dir.clone() + "data.yaml").unwrap();
    data_yaml_file.write(b"train: ./bdd100k_formatted_dataset/train/images\n").unwrap();
    data_yaml_file.write(b"val: ./bdd100k_formatted_dataset/val/images\n").unwrap();
    data_yaml_file.write(b"test: ./bdd100k_formatted_dataset/test/images\n").unwrap();
    data_yaml_file.write(b"\n").unwrap();
    data_yaml_file.write(b"nc: 10\n").unwrap();
    data_yaml_file.write(b"names: ['pedestrian', 'rider', 'car', 'truck', 'bus', 'train', 'motorcycle', 'bicycle', 'traffic light', 'traffic sign']").unwrap();
}

fn convert_coordinates_to_yolo_format(box_coordinates: HashMap<String, f64>) -> HashMap<String, f64> {
    // get box coordinates
    let x1 = box_coordinates["x1"];
    let y1 = box_coordinates["y1"];
    let x2 = box_coordinates["x2"];
    let y2 = box_coordinates["y2"];

    // get box width and height
    let box_width = x2 - x1;
    let box_height = y2 - y1;

    // get box center x and y
    let box_center_x = x1 + (box_width / 2.0);
    let box_center_y = y1 + (box_height / 2.0);

    // normalize box center x and y
    let normalized_box_center_x = normalize_coordinate_value(box_center_x, IMAGE_SIZE.0);
    let normalized_box_center_y = normalize_coordinate_value(box_center_y, IMAGE_SIZE.1);

    // normalize box width and height
    let normalized_box_width = normalize_coordinate_value(box_width, IMAGE_SIZE.0);
    let normalized_box_height = normalize_coordinate_value(box_height, IMAGE_SIZE.1);

    // create new map
    let mut normalized_box_coordinates: HashMap<String, f64> = HashMap::new();

    // add normalized box center x and y to map
    normalized_box_coordinates.insert("x".to_string(), normalized_box_center_x);
    normalized_box_coordinates.insert("y".to_string(), normalized_box_center_y);
    normalized_box_coordinates.insert("w".to_string(), normalized_box_width);
    normalized_box_coordinates.insert("h".to_string(), normalized_box_height);

    // return map
    return normalized_box_coordinates;
}

fn normalize_coordinate_value(value: f64, image_size: f64) -> f64 {
    return value / image_size;
}

fn get_object_class_and_box_coordinates_json(object: &Value) -> (i8, HashMap<String, f64>) {
    // get object class
    let object_class = object["category"].as_str().unwrap().to_lowercase();

    println!("Object class: {}", object_class);

    // make sure object class is in object classes array
    if !OBJECT_CLASSES.contains(&object_class.as_str()) {
        println!("Object class not in object classes array");
        return (-1, HashMap::new());
    }

    // get the index of the object class from the object classes array
    let object_class_index = OBJECT_CLASSES.iter().position(|&r| r == object_class).unwrap() as i8;

    // get object box coordinates and add to map
    let box_coordinates: HashMap<String, f64> = object["box2d"].as_object().unwrap().iter().map(|(k, v)| (k.clone(), v.as_f64().unwrap())).collect();

    // format and normalize box coordinates into yolo format
    let normalized_box_coordinates = convert_coordinates_to_yolo_format(box_coordinates);

    // return object class and box coordinates
    return (object_class_index, normalized_box_coordinates);
}

fn get_objects_in_image(json_object: &Value) -> Vec<String> {
    // try catch get objects in image
    let objects_in_image = json_object["labels"].as_array();

    if !json_object["labels"].is_array() {
        return Vec::new();
    }

    let objects_in_image = objects_in_image.unwrap();

    // output array
    let mut objects_in_image_output: Vec<String> = Vec::new();

    // loop through objects in image
    for object in objects_in_image {
        // check if object is an object class and box coordinates - check for category and box2d keys
        if !object["category"].is_string() || !object["box2d"].is_object() {
            println!("Object is not an object class and box coordinates");
            continue;
        }

        // get object class and box coordinates
        let (object_class, box_coordinates) = get_object_class_and_box_coordinates_json(object);

        if object_class == -1 {
            continue;
        }

        println!("Object class: {}", object_class);
        println!("Box coordinates: {:?}", box_coordinates);

        // add object class and box coordinates to output array
        objects_in_image_output.push(format!("{} {} {} {} {}\n", object_class, box_coordinates["x"], box_coordinates["y"], box_coordinates["w"], box_coordinates["h"]));
    }


    return objects_in_image_output;
}

fn create_label_txt_file(image_name: String, set: String, new_dir: String, json_object: Value) {
    // get objects in image
    let objects_in_image = get_objects_in_image(&json_object);

    // remove image name extension
    let image_name = image_name.replace(".jpg", "");

    // create label txt file
    let mut label_txt_file = fs::File::create(new_dir.clone() + &set + "/labels/" + &image_name + ".txt").unwrap();

    // loop through objects in image
    for object in objects_in_image {
        // write object class and box coordinates to label txt file
        label_txt_file.write(object.as_bytes()).unwrap();
    }
}

fn copy_image_and_create_label(image_name: String, set: String, new_dir: String, json_object: Value) {
    copy_image(new_dir.clone(), image_name.clone(), set.clone());

    // create label txt file
    create_label_txt_file(image_name.clone(), set.clone(), new_dir.clone(), json_object.clone());
}


fn create_train_val_test_sets(matched_names: Vec<String>) {
    // create train, val and test sets
    let len = matched_names.len();
    let (train_set, rest) = matched_names.split_at((len as f64 * 0.8) as usize);
    let (val_set, test_set) = rest.split_at((rest.len() as f64 * 0.5) as usize);

    println!("Number of train images: {}", train_set.len());
    println!("Number of val images: {}", val_set.len());
    println!("Number of test images: {}", test_set.len());

    // create train, val and test folders
    let new_dir: String = "./bdd100k_formatted_dataset/".to_string();
    create_folder_structure(new_dir.clone());

    // create data yaml file
    create_data_yaml(new_dir.clone());

    // open json files
    match fs::read_dir(LABELS_PATH){
        Ok(entries) => {
            for entry in entries {
                // open json file
                let file = fs::File::open(entry.unwrap().path()).unwrap();
                let reader = io::BufReader::new(file);

                // parse json file
                let json: Value = serde_json::from_reader(reader).unwrap();

                // loop through json file
                for object in json.as_array().unwrap() {
                    // get image name
                    let image_name = object["name"].as_str().unwrap().to_string();

                    // if image name is in train set
                    if train_set.contains(&image_name) {
                        // copy and create label file
                        copy_image_and_create_label(image_name.clone(), "train".to_string(), new_dir.clone(), object.clone());
                    }

                    // if image name is in val set
                    if val_set.contains(&image_name) {
                        // copt and create label file
                        copy_image_and_create_label(image_name.clone(), "val".to_string(), new_dir.clone(), object.clone());
                    }

                    // if image name is in test set
                    if test_set.contains(&image_name) {
                        // copy and create label file
                        copy_image_and_create_label(image_name.clone(), "test".to_string(), new_dir.clone(), object.clone());
                    }
                }
            }
        },
        Err(e) => println!("Error reading directory: {}", e),
    }

    println!("Finished");
}