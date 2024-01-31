import os
import shutil
import random

SOURCE_720_PATH = "F:/Dissertation/Tests/Performance Test Sets/720"
SOURCE_1080_PATH = "F:/Dissertation/Recorded_driving/Frames"
DESTINATION_PATH = "F:/Dissertation/Tests/Performance Test Sets/Mixed"
TEST_SET_SIZE = 8000

def main():
    print("Generating performance test set...")
    print("Test set size: " + str(TEST_SET_SIZE))
    print("Source 720 path: " + SOURCE_720_PATH)
    print("Source 1080 path: " + SOURCE_1080_PATH)
    print("Destination path: " + DESTINATION_PATH)

    print("Creating folder file dictionaries...")
    file_list_720 = create_720_file_list()
    folder_file_dict_1080 = create_1080_folder_file_dict()

    print("Generating test set...")
    resolution_count = {"720": 0, "1080": 0}
    file_tracker = []
    for i in range(0, TEST_SET_SIZE):
        print("Generating image " + str(i + 1) + "...")
        # randomly choose 720 or 1080
        random_resolution = random.choice([720, 1080])

        if random_resolution == 720:
            resolution_count["720"] += 1
        else:
            resolution_count["1080"] += 1

        # randomly select a folder
        if random_resolution == 1080:
            random_folder = random.choice(list(folder_file_dict_1080.keys()))
        # randomly select a file from the folder
        while True:
            if random_resolution == 720:
                random_file = random.choice(file_list_720)
            else:
                random_file = random.choice(folder_file_dict_1080[random_folder])

            if random_file not in file_tracker:
                file_tracker.append(random_file)
                break
            else:
                print("File already in test set, selecting another...")
                continue

        # copy file to destination path
        if random_resolution == 720:
            shutil.copy(SOURCE_720_PATH + "/" + random_file, DESTINATION_PATH + "/" + random_file)
        else:
            shutil.copy(SOURCE_1080_PATH + "/" + random_folder + "/" + random_file, DESTINATION_PATH + "/" + random_file)

    print("Performance test set generated for mixed resolution.")
    print("720 Samples: " + str(resolution_count["720"]))
    print("1080 Samples: " + str(resolution_count["1080"]))

def create_1080_folder_file_dict():
    folder_file_dict = {}

    # get folders in source path
    folders = os.listdir(SOURCE_1080_PATH)

    # for each folder, get all files
    for folder in folders:
        files = os.listdir(SOURCE_1080_PATH + "/" + folder)
        folder_file_dict[folder] = files
        print("Folder " + folder + " has " + str(len(files)) + " files.")

    return folder_file_dict

def create_720_file_list():
    # get files in source path
    files = os.listdir(SOURCE_720_PATH)
    print("Folder 720 has " + str(len(files)) + " files.")

    return files

main()