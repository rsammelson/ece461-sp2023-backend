'''Python script implementation of fraction dependencies metric.'''

import os
import json
import sys

def find_deps(repo_identifier):
    '''Finding dependencies in repo'''
    #check slashes
    #file = '$HOME/.cache/acme'
    file = os.getcwd()

    # print(repo_identifier)

    owner_index = repo_identifier.find('/')
    repo_owner = repo_identifier[0 : owner_index]

    repo_identifier = repo_identifier[(owner_index+1):]
    # end_index = min(repo_identifier.find('\r'), repo_identifier.find('\n'))

    # repo_name = repo_identifier[:end_index]
    repo_name = repo_identifier
    #print(repo_name, end='')

    file = file + '/acme/' + repo_owner + '/' + repo_name + '/' + 'package.json'

    #print("\n\n\n",file,"\n\n\n")

    exist = os.path.isfile(file)
    #print("Returning... ")

    if exist:
        with open(file, encoding="utf-8") as json_file:
            data = json.load(json_file)
            if not "dependencies" in data.keys():
                print("1")
                return 0 # no dependencies
            count = len(data["dependencies"])
            print(count)
            return 0
    else:
        print("-1")
        return 0

if find_deps(sys.argv[1]) == -1:
    sys.exit(1)
else:
    sys.exit(0)
