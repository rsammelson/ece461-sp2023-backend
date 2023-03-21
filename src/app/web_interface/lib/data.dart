import 'main.dart' show columns;

class PackageRegistry {
  // Make this class have only a single instance
  // This way can do PackageRegistry() to access same class anywhere
  // Prevents mixed data values from different instances when used across app
  static final PackageRegistry _instance = PackageRegistry._internal();

  // internal vars
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
      ["1", "package name 1", "1.1.1", "OK"],
      ["2", "package name 2", "1.0.2.7+1", "OK"],
      ["3", "package name 3", "3.7.2", "OK"],
      ["4", "package name 4", "5.6", "OK"],
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
      return true;
    } else if (curSortMethod == columns[1]) {
      return true;
    } else if (curSortMethod == columns[2]) {
      return true;
    } else if (curSortMethod == columns[3]) {
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
