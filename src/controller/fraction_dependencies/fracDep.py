#Locate cloned repo
##API->Fetch->mod.rs
##$HOME/.cache/acme/{repo_owner}/{repo_name}

#Locate package.json

#Open package.json

#Count dependencies

#NEED TO ADD DEPENDENCY?
import os
import json

file = '$HOME/.cache/acme'

def findDeps(repo_owner, repo_name):
    #check slashes
    file = file + '/' + repo_owner + '/' + repo_name + '/' + 'package.json'
    exist = os.path.isfile(file)

    if exist:
        with open('file') as json_file:
            data = json.load(json_file)
        count = len(data["devDependencies"])
        return count

    else:
        print("error, no path")
        return -1
