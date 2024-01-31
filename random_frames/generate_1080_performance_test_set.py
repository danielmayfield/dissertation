import os
import shutil
import random

SOURCE_PATH = "F:/Dissertation/Recorded_driving/Frames"
DESTINATION_PATH = "F:/Dissertation/Tests/Performance Test Sets/1080"
TEST_SET_SIZE = 8000

def main():
    print("Generating performance test set...")
    print("Test set size: " + str(TEST_SET_SIZE))
    print("Source path: " + SOURCE_PATH)
    print("Destination path: " + DESTINATION_PATH)

    print("Creating folder file dictionary...")
    folder_file_dict = create_folder_file_dict()

    print("Generating test set...")
    file_tracker = []
    for i in range(0, TEST_SET_SIZE):
        print("Generating image " + str(i + 1) + "...")
        # folder structure is folder for each video frames
        # randomly select a folder
        random_folder = random.choice(list(folder_file_dict.keys()))
        # randomly select a file from the folder
        while True:
            random_file = random.choice(folder_file_dict[random_folder])
            if random_file not in file_tracker:
                file_tracker.append(random_file)
                break
            else:
                print("File already in test set, selecting another...")
                continue

        # copy file to destination path
        shutil.copy(SOURCE_PATH + "/" + random_folder + "/" + random_file, DESTINATION_PATH + "/" + random_file)

    print("Performance test set generated for 1080 resolution.")

def create_folder_file_dict():
    folder_file_dict = {}

    # get folders in source path
    folders = os.listdir(SOURCE_PATH)

    # for each folder, get all files
    for folder in folders:
        files = os.listdir(SOURCE_PATH + "/" + folder)
        folder_file_dict[folder] = files
        print("Folder " + folder + " has " + str(len(files)) + " files.")

    return folder_file_dict

main()