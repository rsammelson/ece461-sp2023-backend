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

###########################################
def is_valid_url(url):
    try:
        result = urlparse(url)
        return all([result.scheme, result.netloc])
    except ValueError:
        return False

###########################################
def compress_folder(input_folder, output_zip):
    # Open the output ZIP file in write mode
    with zipfile.ZipFile(output_zip, 'w', zipfile.ZIP_DEFLATED) as zipf:
        # Walk through the input folder
        for root, dirs, files in os.walk(input_folder):
            for file in files:
                # Get the file path relative to the input folder
                rel_path = os.path.relpath(os.path.join(root, file), input_folder)
                # Add the file to the ZIP file with its relative path
                zipf.write(os.path.join(root, file), arcname=rel_path)

###########################################
def grab_content(repo_path):
    """
    Compress a GitHub repository at the specified path into a zip file,
    and encode the zip file as a base64 string.
    """
    # Create a temporary file to hold the compressed repository
    zip_path = "/tmp/repo.zip"
    
    print(repo_path)

    os.system("rm -rf {}".format(repo_path + "/.git"))

    # Compress the repository into a zip file
    #with zipfile.ZipFile(zip_path, 'w', zipfile.ZIP_DEFLATED) as zip_file:
       # for root, dirs, files in os.walk(repo_path):
           # for file in files:
               # file_path = os.path.join(root, file)
               # zip_file.write(file_path, arcname=file)

    compress_folder(repo_path, zip_path)

    # Encode the zip file as a base64 string
    with open(zip_path, "rb") as f:
        encoded_string = base64.b64encode(f.read()).decode("utf-8")

    # Delete the temporary file
    os.remove(zip_path) #HEREEEEEEEEEEEEEEEEE

    return encoded_string

#############################################
def decode_base64(base64_string):

    # Making sure acme directory is clear
    acme_dir = os.getcwd() + '/ECE461-Project-ACMEIR/acme'
    shutil.rmtree(acme_dir)
    os.makedirs(acme_dir)
    
    # Decode the base64 string
    # Decode the base64 string and create a BytesIO object
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
            # Write the file to disk
            if filename:
                # print(filename)
                with zip_file.open(file_info) as file:
                    content = file.read()
                    # print(content)
                    with open(os.path.join("extracted_files", file_info.filename), mode='wb') as new_file:
                        new_file.write(content)

    # Current path plus directory with repo
    path_ = cwd + '/extracted_files'

    # Step into a directory
    os.chdir(path_)

    # Get the list of directories in the current directory
    directories = os.listdir()

    # Loop over the directories to find the one you want to step into
    for directory in directories:
        # Check if the current item is a directory
        if os.path.isdir(directory):
            # If it is, step into the directory
            os.chdir(directory)
            #print(directory)
            break  # Stop looping once you've found the directory
        
    # Current path plus package.son file
    file = os.getcwd() + '/' + 'package.json'
    
    #Checking package.json exits
    exist = os.path.isfile(file)

    #For later
    #name = ""
    #version = ""
    url = ""

    if exist:
        with open(file, encoding="utf-8") as json_file:
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
def decode(input_string, jsprogram):
#need to grab name and version

    cwd = os.getcwd()

    url = input_string

    url_given = True

    content = input_string

    if not is_valid_url(input_string):
        url = decode_base64(input_string)
        url_given = False

    #Moving into ECE461-Project-ACMEIR.
    os.chdir(cwd + '/ECE461-Project-ACMEIR')

    # Open a text file for writing
    with open('URL_FILE', 'w') as file:
        # Write some text to the file
        file.write(url)

    # Target we want to run in Makefile
    target = "runmain"

    # Run make command with a target
    result = subprocess.run(["bash", "runmain.sh", "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"], capture_output=True, text=True)

    metrics = {}
    file_contents = result.stdout
    metrics = json.loads(file_contents)

    os.chdir(cwd + '/ECE461-Project-ACMEIR/acme')

    path_ = ""

    # Get the list of directories in the current directory
    directories = os.listdir()

    #LOOP INTO USERNAME DIRECTORY
    for directory in directories:
        # Check if the current item is a directory
        if os.path.isdir(directory):
            # If it is, step into the directory
            path_ = os.getcwd() + '/' + directory
            #print(path_)
            os.chdir(path_)

            directories2 = os.listdir()
            for directory2 in directories2:
                
                if os.path.isdir(directory2):
                    
                    path_2 = os.getcwd() + '/' + directory2

                    if url_given:
                       content = grab_content(path_2)
                       #content = compress_zip_and_base64(path_2)

                    os.chdir(path_2)
                    
                    break

            break  # Stop looping once you've found the directory

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

    # taken from https://www.geeksforgeeks.org/python-generate-random-string-of-given-length/
    id = str(''.join(random.choices(string.ascii_uppercase +
                             string.digits, k=20)))
    # used https://stackoverflow.com/questions/2150739/iso-time-iso-8601-in-python for utc iso date
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

