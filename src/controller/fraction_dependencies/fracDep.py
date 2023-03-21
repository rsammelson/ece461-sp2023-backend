import os
import json

file = '$HOME/.cache/acme'

def findDeps(repo_identifier):
    #check slashes
    
    owner_index = repo_identifier.find('/')
    repo_owner = repo_identifier[0 : owner_index]

    repo_name = repo_identifier[(owner_index + 1) :]

    file = file + '/' + repo_owner + '/' + repo_name + '/' + 'package.json'
    exist = os.path.isfile(file)

    if exist:
        with open('file') as json_file:
            data = json.load(json_file)
        count = len(data["dependencies"])
        return count

    else:
        print("error, no path")
        return -1
