import 'package:cloud_firestore/cloud_firestore.dart';

import 'main.dart' show columns;

class PackageRegistry {
  // Make this class have only a single instance
  // This way can do PackageRegistry() to access same class anywhere
  // Prevents mixed data values from different instances when used across app
  static final PackageRegistry _instance = PackageRegistry._internal();

  // vars (leading _ means internal)
  bool isSortAscending = true;
  String curSortMethod = columns[0];
  List<Map<String, dynamic>>? _data;
  List<Map<String, dynamic>> selectedData = [];

  // factory will return an instance, not necessarily creating a new one
  factory PackageRegistry() {
    return _instance;
  }

  PackageRegistry._internal() {
    // initialization logic

    // data = grabData();
    _data = [
      {
        "name": "Package",
        "rating": 1.5,
        "id": 1,
        "info": "Extra desc",
        "version": "1.5.6+1",
        "url": "https://console.firebase.google.com"
      },
    ];

    // format data on init
    formatData();
  }

  // rest of class as normal

  List<Map<String, dynamic>> get data => searchData('');

  set data(List<Map<String, dynamic>> values) => _data = values;

  bool formatData() {
    if (_data == null || _data!.isEmpty) {
      return false;
    }

    bool didFormat = false;
    for (int i = 0; i < _data!.length; i++) {
      // format mapping?
    }
    return didFormat;
  }

  Future<bool> importData() async {
    // Grab data stored in the cloud and set data value of this class
    // FirebaseFirestore.instance.collection(collectionPath)
    List<Map<String, dynamic>> newData = [];
    var firestoreData =
        await FirebaseFirestore.instance.collection('/packages').get();
    for (var ff in firestoreData.docs) {
      newData.add(ff.data());
    }
    _data = newData;

    return false;
  }

  List<List<dynamic>> grabData() {
    // Grab data stored in the cloud and return it
    return [];
  }

  bool sortData() {
    if (_data == null || _data!.isEmpty) {
      return false;
    }

    if (curSortMethod == columns[0]) {
      _data!.sort(
        (a, b) => isSortAscending
            ? int.parse(a['id']).compareTo(int.parse(b['id']))
            : int.parse(b['id']).compareTo(int.parse(a['id'])),
      );
      return true;
    } else if (curSortMethod == columns[1]) {
      _data!.sort(
        (a, b) => isSortAscending
            ? '${a['name']}'
                .toLowerCase()
                .compareTo('${b['name']}'.toLowerCase())
            : '${b['name']}'
                .toLowerCase()
                .compareTo('${a['name']}'.toLowerCase()),
      );
      return true;
    } else if (curSortMethod == columns[2]) {
      _data!.sort(
        (a, b) {
          // split 1.0.0 into ['1', '0', '0']
          List<String> firstVersions = '${a['version']}'.split(".");
          List<String> secondVersions = '${b['version']}'.split(".");

          // choose the greater of two lengths
          int numCompares = firstVersions.length > secondVersions.length
              ? firstVersions.length
              : secondVersions.length;
          for (var i = 0; i <= numCompares; i++) {
            try {
              try {
                int compare = isSortAscending
                    ? int.parse(firstVersions[i]) - int.parse(secondVersions[i])
                    : int.parse(secondVersions[i]) -
                        int.parse(firstVersions[i]);
                if (compare != 0) {
                  return compare;
                }
              } on IndexError {
                // If two exact same versions but one is longer
                // Such as 3.7.2 and 3.7
                return isSortAscending
                    ? firstVersions.length - secondVersions.length
                    : secondVersions.length - firstVersions.length;
              }
            } on FormatException {
              // If version of form 1.0.0+1, int.parse() will fail
              // Therefore, compare the x value and the y value in 1.0.x+y
              List<String> x = firstVersions[i].split("+");
              List<String> y = secondVersions[i].split("+");
              int compare = isSortAscending
                  ? int.parse(x[0]) - int.parse(y[0])
                  : int.parse(y[0]) - int.parse(x[0]);
              if (compare != 0) {
                return compare;
              } else {
                compare = isSortAscending
                    ? int.parse(x[1]) - int.parse(y[1])
                    : int.parse(y[1]) - int.parse(x[1]);
                return compare;
              }
            }
          }
          // in case of error, return them as equal
          return 0;
        },
      );
      return true;
    } else if (curSortMethod == columns[3]) {
      _data!.sort(
        (a, b) => isSortAscending
            ? '${a[3]}'.toLowerCase().compareTo('${b['rating']}'.toLowerCase())
            : '${b[3]}'.toLowerCase().compareTo('${a['rating']}'.toLowerCase()),
      );
      return true;
    } else {
      return false;
    }
  }

  List<Map<String, dynamic>> searchData(String keyword) {
    if (_data == null || _data!.isEmpty) {
      return [];
    } else if (keyword == '') {
      return _data!;
    }

    List<Map<String, dynamic>> filtered = [];

    for (Map<String, dynamic> row in _data!) {
      if ('${row['name']}'.toLowerCase().contains(keyword)) {
        filtered.add(row);
      }
    }

    return filtered;
  }
}
