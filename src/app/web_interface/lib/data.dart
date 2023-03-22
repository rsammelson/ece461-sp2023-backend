import 'package:flutter/foundation.dart';

import 'main.dart' show columns;

class PackageRegistry {
  // Make this class have only a single instance
  // This way can do PackageRegistry() to access same class anywhere
  // Prevents mixed data values from different instances when used across app
  static final PackageRegistry _instance = PackageRegistry._internal();

  // vars (leading _ means internal)
  bool isSortAscending = true;
  String curSortMethod = columns[0];
  List<List<dynamic>>? _data;
  List<List<dynamic>> selectedData = [];

  // factory will return an instance, not necessarily creating a new one
  factory PackageRegistry() {
    return _instance;
  }

  PackageRegistry._internal() {
    // initialization logic

    // data = grabData();
    _data = [
      ["1", "Example 2", "1.1.1", "OK"],
      ["2", "Example 1", "1.0.2.7+1", "OK"],
      ["3", "Fluent UI", "3.7.2", "OK"],
      ["4", "Flutter", "5.6", "OK"],
      ["5", "My Package", "1.0.2.7+2", "OK"],
      ["6", "Flutter", "3.7", "OK"],
    ];

    // format data on init
    formatData();
  }

  // rest of class as normal

  List<List<dynamic>> get data => searchData('');

  set data(List<List<dynamic>> values) => _data = values;

  bool formatData() {
    if (_data == null || _data!.isEmpty) {
      return false;
    }

    bool didFormat = false;
    for (int i = 0; i < _data!.length; i++) {
      for (int j = 0; j < _data![i].length; j++) {
        if (_data![i][j] == "") {
          _data![i][j] = "--";
          didFormat = true;
        }
      }
    }
    return didFormat;
  }

  bool importData() {
    // Grab data stored in the cloud and set data value of this class
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
            ? int.parse(a[0]).compareTo(int.parse(b[0]))
            : int.parse(b[0]).compareTo(int.parse(a[0])),
      );
      return true;
    } else if (curSortMethod == columns[1]) {
      _data!.sort(
        (a, b) => isSortAscending
            ? '${a[1]}'.toLowerCase().compareTo('${b[1]}'.toLowerCase())
            : '${b[1]}'.toLowerCase().compareTo('${a[1]}'.toLowerCase()),
      );
      return true;
    } else if (curSortMethod == columns[2]) {
      _data!.sort(
        (a, b) {
          // split 1.0.0 into ['1', '0', '0']
          List<String> firstVersions = '${a[2]}'.split(".");
          List<String> secondVersions = '${b[2]}'.split(".");

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
              } on IndexError catch (exception) {
                // If two exact same versions but one is longer
                // Such as 3.7.2 and 3.7
                return isSortAscending
                    ? firstVersions.length - secondVersions.length
                    : secondVersions.length - firstVersions.length;
              }
            } on FormatException catch (exception) {
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
            ? '${a[3]}'.toLowerCase().compareTo('${b[3]}'.toLowerCase())
            : '${b[3]}'.toLowerCase().compareTo('${a[3]}'.toLowerCase()),
      );
      return true;
    } else {
      return false;
    }
  }

  List<List<dynamic>> searchData(String keyword) {
    List<List<dynamic>> filtered = [];
    if (_data == null || _data!.isEmpty) {
      return filtered;
    } else if (keyword == '') {
      return _data!;
    }

    for (List<dynamic> row in _data!) {
      if ('${row[1]}'.contains(keyword)) {
        filtered.add(row);
      }
    }

    return filtered;
  }
}
