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
  List<Map<String, dynamic>> filteredData = [];

  // factory will return an instance, not necessarily creating a new one
  factory PackageRegistry() {
    return _instance;
  }

  PackageRegistry._internal() {
    // initialization logic
    _data = [];
  }

  // rest of class as normal

  List<Map<String, dynamic>> get data => searchData('');

  set data(List<Map<String, dynamic>> values) => _data = values;

  Future<bool> importData() async {
    // Grab data stored in the cloud and set data value of this class
    //
    List<Map<String, dynamic>> newData = await grabData();
    _data = newData;
    filteredData = newData;

    return newData.isNotEmpty;
  }

  Future<List<Map<String, dynamic>>> grabData() async {
    // Grab data stored in the cloud and return it
    //
    List<Map<String, dynamic>> newData = [];
    // Get query snapshot of collection 'packages'
    try {
      QuerySnapshot<Map<String, dynamic>> firestoreDataSnapshot =
          await FirebaseFirestore.instance.collection('/packages').get();
      // Query snapshot .docs method returns a list of query snapshots of every document collection
      // For every document query snapshot, take package data as mapping and add mapping to list of data
      for (QueryDocumentSnapshot<Map<String, dynamic>> docSnapshot
          in firestoreDataSnapshot.docs) {
        newData.add(docSnapshot.data());
      }
      return newData;
    } catch (e) {
      // usually = permission-denied
      return [];
    }
  }

  bool sortData() {
    // Sort data based on variables that were set
    // isSortAscending
    // curSortMethod

    // If no data, nothing to sort
    if (_data == null || _data!.isEmpty) {
      return false;
    }

    // Decide which column to sort
    if (curSortMethod == columns[0]) {
      _data!.sort(
        (a, b) => isSortAscending
            ? int.parse('${a['id']}').compareTo(int.parse('${b['id']}'))
            : int.parse('${b['id']}').compareTo(int.parse('${a['id']}')),
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
            ? '${a['rating']}'
                .toLowerCase()
                .compareTo('${b['rating']}'.toLowerCase())
            : '${b['rating']}'
                .toLowerCase()
                .compareTo('${a['rating']}'.toLowerCase()),
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

    // Search name category of all data to see if regardless of capitalization does the keyword show up anywhere in the package name
    for (Map<String, dynamic> row in _data!) {
      if ('${row['name']}'.toLowerCase().contains(keyword)) {
        filtered.add(row);
      }
    }

    return filtered;
  }
}
