import os
import json
import sys


def findDeps(repo_identifier):
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
        with open(file) as json_file:
            data = json.load(json_file)
            if not("dependencies" in data.keys()):
                print("1")
                return 0 # no dependencies
            count = len(data["dependencies"])
            print(count)
            #print("111")
            return 0
        print("David")
    else:
        #print("error, no path")
        print("-1")
        return 0
        

if findDeps(sys.argv[1]) == -1:
    exit(1)
else:
    exit(0)
