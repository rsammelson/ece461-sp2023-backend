import base64
import zipfile
import os
import io
import shutil
import json
import subprocess
import sys
import requests
import random
import string
import datetime
from urllib.parse import urlparse

# ChatGPT was referenced to help develop 
# parts of this code

###########################################
# Checks if url is valid
def is_valid_url(url):
    try:
        result = urlparse(url)
        return all([result.scheme, result.netloc])
    except ValueError:
        return False

###########################################
# Compresses and zips folder
def compress_folder(input_folder, output_zip):
    # Open the output ZIP file in write mode
    with zipfile.ZipFile(output_zip, 'w', zipfile.ZIP_DEFLATED) as zipf:
        # Step through input folder
        for root, dirs, files in os.walk(input_folder):
            for file in files:
                # Get the file path relative to the input folder
                rel_path = os.path.relpath(os.path.join(root, file), input_folder)
                # Add the file to the ZIP file with its relative path
                zipf.write(os.path.join(root, file), arcname=rel_path)

###########################################
# Zips a repo and converts it to base64 string
def grab_content(repo_path):

    # Temporary zip folder to hold the compressed repo
    zip_path = "/tmp/repo.zip"
    
    # Delete the .git file
    os.system("rm -rf {}".format(repo_path + "/.git"))

    # Compress and zip repo
    compress_folder(repo_path, zip_path)

    # Encode zip file as a base64 string
    with open(zip_path, "rb") as f:
        encoded_string = base64.b64encode(f.read()).decode("utf-8")

    # Delete the temp folder
    os.remove(zip_path)

    return encoded_string

#############################################
# Takes a base64 encoded zip file, and decodes it
# to grab and return the repo URL
def decode_base64(base64_string):

    # Making sure acme directory is clear
    acme_dir = os.getcwd() + '/ECE461-Project-ACMEIR/acme'
    shutil.rmtree(acme_dir)
    os.makedirs(acme_dir)
    
    # Decode the base64 string 
    decoded_data = base64.b64decode(base64_string)
    bytes_io = io.BytesIO(decoded_data)

    cwd = os.getcwd()

    # Create a ZipFile object from the BytesIO object
    with zipfile.ZipFile(bytes_io, mode='r') as zip_file:
        # Extract files from the zip file
        for file_info in zip_file.infolist():
            # Get the directory and filename of the file
            dirname, filename = os.path.split(file_info.filename)
            # Create the directory if it doesn't exist
            if dirname and not os.path.exists(os.path.join("extracted_files", dirname)):
                os.makedirs(os.path.join("extracted_files", dirname))
            # Write to file
            if filename:
                # print(filename)
                with zip_file.open(file_info) as file:
                    content = file.read()
                    # print(content)
                    with open(os.path.join("extracted_files", file_info.filename), mode='wb') as new_file:
                        new_file.write(content)

    # Current path plus directory with repo
    path_ = cwd + '/extracted_files'

    # Step into directory
    os.chdir(path_)

    # Get the list of directories in the current directory
    directories = os.listdir()

    # Loop over the directories (should only be one)
    for directory in directories:
        # Check if the current item is a directory
        if os.path.isdir(directory):
            # Step into the directory
            os.chdir(directory)
            #print(directory)
            break
        
    # Current path plus package.son file
    file = os.getcwd() + '/' + 'package.json'
    
    #Checking package.json exits
    exist = os.path.isfile(file)

    # Creating URL string
    url = ""

    # Open package.json if exists
    if exist:
        # Open file
        with open(file, encoding="utf-8") as json_file:
            # Collect .json data
            data = json.load(json_file)
            if not "repository" in data.keys():
                print("ERROR: NO URL") 
            url = data["repository"]["url"]
            #name = data["name"]
            #version = data["version"]
    else:
        print("ERROR: NO package.json")

    # Step out of extracted_files
    os.chdir(cwd)

    # Delete extracted files
    shutil.rmtree(path_)

    return url

#############################################
# "main" function of this .py file
# Called by call_decode.py
# Decodes and encodes base64 zips
# Runs part 1 code
# Sends outputs to API
def decode(input_string, jsprogram):

    cwd = os.getcwd()
    
    # Assuming input is url 
    url = input_string
    url_given = True

    # Assuming input is base64 content
    content = input_string

    # Check if input_string is url or base64 content
    if not is_valid_url(input_string):
        # Grab repo URL for part 1 code
        url = decode_base64(input_string)
        url_given = False

    #Moving into ECE461-Project-ACMEIR - part 1 codebase
    os.chdir(cwd + '/ECE461-Project-ACMEIR')

    # Open a URL_FILE text file for part 1 code
    with open('URL_FILE', 'w') as file:
        file.write(url)

    # Target to run in Makefile
    #target = "runmain"

    # Run bash file
    result = subprocess.run(["bash", "runmain.sh", "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"], capture_output=True, text=True)

    # Collecting part 1 code metric outputs
    metrics = {}
    file_contents = result.stdout
    metrics = json.loads(file_contents)

    os.chdir(cwd + '/ECE461-Project-ACMEIR/acme')

    path_ = ""

    # Get the list of directories (GITHUB USERNAMES)
    directories = os.listdir()

    #Loop into directory (GITHUB USERNAMES)
    for directory in directories:
        # Check if the current item is a directory
        if os.path.isdir(directory):
            # Step into the directory
            path_ = os.getcwd() + '/' + directory
            #print(path_)
            os.chdir(path_)
            
            # Get directories (REPO NAMES)
            directories2 = os.listdir()
            
            # Loop through REPO NAMES
            for directory2 in directories2:
                
                if os.path.isdir(directory2):
                    
                    path_2 = os.getcwd() + '/' + directory2

                    if url_given:
                        # Grab base64 encoded content
                       content = grab_content(path_2)

                    os.chdir(path_2)
                    
                    break

            break

    # Check if package.json exists to grab data
    file = path_2 + '/package.json'
    exist = os.path.isfile(file)

    if exist:
        with open(file, encoding="utf-8") as json_file:
            data = json.load(json_file)
            name = data["name"]
            version = data["version"]
    else:
        print("ERROR: NO package.json")

    os.chdir(cwd + '/ECE461-Project-ACMEIR')
    
    # Remove repo clone created by part 1 code
    shutil.rmtree(path_)

    NAME = name
    VERSION = version
    NET_SCORE = str(metrics['NET_SCORE'])
    RAMP_UP_SCORE = str(metrics['RAMP_UP_SCORE'])
    LICENSE_SCORE = str(metrics['LICENSE_SCORE'])
    BUS_FACTOR_SCORE = str(metrics['BUS_FACTOR_SCORE'])
    FRACTION_DEPENDENCIES_SCORE = str(metrics['FRACTION_DEPENDENCIES_SCORE'])
    FRACTION_REVIEWED_SCORE = str(metrics['FRACTION_REVIEWED_SCORE'])
    CORRECTNESS_SCORE = str(metrics['CORRECTNESS_SCORE'])
    RESPONSIVE_MAINTAINER_SCORE = str(metrics['RESPONSIVE_MAINTAINER_SCORE'])

    print(content)

    # Taken from https://www.geeksforgeeks.org/python-generate-random-string-of-given-length/
    id = str(''.join(random.choices(string.ascii_uppercase +
                             string.digits, k=20)))
    # Used https://stackoverflow.com/questions/2150739/iso-time-iso-8601-in-python for utc iso date
    date = datetime.datetime.utcnow().isoformat()
    date = date[0:date.index('.')]+'Z'

    url = "https://firestore.googleapis.com/v1/projects/acme-register/databases/(default)/documents/packages?documentId="+id
    request_body = {
        "fields" : {
            "Name" : {
                "stringValue": NAME,
            },
            "JSProgram" : {
                "stringValue": jsprogram,
            },
            "Version" : {
                "stringValue": VERSION,
            },
            "NetScore" : {
                "stringValue": NET_SCORE,
            },
            "RampUpScore" : {
                "stringValue": RAMP_UP_SCORE,
            },
            "LicenseScore" : {
                "stringValue": LICENSE_SCORE,
            },
            "BusFactorScore" : {
                "stringValue": BUS_FACTOR_SCORE,
            },
            "FractionDependenciesScore" : {
                "stringValue": FRACTION_DEPENDENCIES_SCORE,
            },
            "FractionReviewedScore" : {
                "stringValue": FRACTION_REVIEWED_SCORE,
            },
            "CorrectnessScore" : {
                "stringValue": CORRECTNESS_SCORE,
            },
            "ResponsiveMaintainerScore" : {
                "stringValue": RESPONSIVE_MAINTAINER_SCORE,
            },
            "ID" : {
                "stringValue": id,
            },
            "History" : {
                "arrayValue" : {
                    'values' : [
                        {
                            'mapValue': {
                                'fields': {
                                    'Action': {
                                        'stringValue': 'CREATE'
                                    }, 
                                    'User': {
                                        'mapValue': {
                                            'fields': {
                                                'name': {
                                                    'stringValue': 'UNIMPLEMENTED'
                                                }, 
                                                'isAdmin': {
                                                    'booleanValue': True
                                                }
                                            }
                                        }
                                    }, 
                                    'Date': {
                                        'stringValue': date
                                    }
                                }
                            }
                        }
                    ]
                }
            }
        }
    }
    print(requests.post(url, data=json.dumps(request_body)).text)

    # https://cloud.google.com/storage/docs/uploading-objects#rest-upload-objects
    url = "https://storage.googleapis.com/upload/storage/v1/b/acme-register-contents/o?uploadType=media&name="+id

    # https://www.w3schools.com/python/ref_requests_post.asp
    response = requests.post(url, data=content)

    # Delete a file
    os.remove('URL_FILE')

    return metrics

