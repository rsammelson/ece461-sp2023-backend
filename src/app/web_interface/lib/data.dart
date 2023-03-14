void getRegistryData() {}

class PackageRegistry {
  PackageRegistry({List<List<dynamic>>? data});
  List<List<dynamic>>? data = [
    ["1", "package name", "", "OK"],
    ["2", "package name", "new", "OK"],
    ["3", "package name", "--", "OK"]
  ];

  bool formatData() {
    bool didFormat = false;
    for (int i = 0; i < data!.length; i++) {
      for (int j = 0; j < data![i].length; j++) {
        if (data![i][j] == "") {
          data![i][j] = "--";
          didFormat = true;
        }
      }
    }
    return didFormat;
  }

  bool grabData() {
    // Grab data stored in the cloud
    return false;
  }

  List<List<dynamic>>? sortBy(
      {required String sortMethod, bool? sortAscending = false}) {
    switch (sortMethod) {
      case 'id':
        break;
      case 'Name':
        break;
      case 'Status':
        break;
      default:
    }
    return data;
  }
  // List<List<dynamic>> get data => ;
}
