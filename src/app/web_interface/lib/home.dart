import 'package:fluent_ui/fluent_ui.dart';
import 'package:web_interface/data.dart';
import 'package:web_interface/popup.dart';

import 'database_table.dart';
import 'main.dart' show columns;

class HomePage extends StatefulWidget {
  const HomePage({
    super.key,
  });

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  final TextEditingController _searchController = TextEditingController();
  final PackageRegistry _pr = PackageRegistry();
  List<List<dynamic>> filteredData = PackageRegistry().data;

  void editSelected(bool addValue, List<dynamic> dataRow) {
    setState(() {
      if (addValue) {
        if (!_pr.selectedData.contains(dataRow)) {
          _pr.selectedData.add(dataRow);
        }
      } else {
        _pr.selectedData.remove(dataRow);
      }
    });
  }

  bool _isAllSelected() {
    for (List<dynamic> row in filteredData) {
      if (!_pr.selectedData.contains(row)) {
        return false;
      }
    }
    return true;
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.center,
      mainAxisAlignment: MainAxisAlignment.start,
      children: [
        //
        // Search bar
        //
        Container(
            constraints: const BoxConstraints(maxWidth: 500),
            padding: const EdgeInsets.symmetric(vertical: 10),
            child: AutoSuggestBox(
              noResultsFoundBuilder: (context) {
                return Container(
                  height: 0,
                );
              },
              onChanged: (text, reason) {
                setState(() {
                  filteredData = _pr.searchData(text);
                });
              },
              controller: _searchController,
              items: const [],
              style: const TextStyle(fontSize: 16),
              clearButtonEnabled: true,
              leadingIcon: const Padding(
                padding: EdgeInsets.all(8),
                child: Icon(FluentIcons.search),
              ),
            )),
        //
        // Command Bar / Filter options
        //
        Padding(
          padding: const EdgeInsets.only(left: 50, right: 50, bottom: 10),
          child: CommandBar(
            mainAxisAlignment: MainAxisAlignment.end,
            overflowBehavior: CommandBarOverflowBehavior.wrap,
            compactBreakpointWidth: 900,
            primaryItems: [
              CommandBarButton(
                  onPressed: () async {
                    // Call add method (make one in PackageRegistry)
                    String result = await showAddPackageDialog(context);
                    setState(() {});
                  },
                  icon: const Icon(FluentIcons.add),
                  label: const Text('Add')),
              CommandBarButton(
                  onPressed: _pr.selectedData.isEmpty
                      ? null
                      : () async {
                          // Call delete method (make one in PackageRegistry)
                          String result = await showDeletePackageDialog(
                              context, _pr.selectedData);
                          setState(() {});
                        },
                  icon: const Icon(FluentIcons.delete),
                  label: Text(
                      'Delete${_pr.selectedData.isEmpty ? '' : ' (${_pr.selectedData.length})'}')),
              CommandBarButton(
                  onPressed: _pr.selectedData.isEmpty
                      ? null
                      : () async {
                          // Call update method (make one in PackageRegistry)
                          String result = await showUpdatePackageDialog(
                              context, _pr.selectedData);
                          setState(() {});
                        },
                  icon: const Icon(FluentIcons.update_restore),
                  label: Text(
                      'Update${_pr.selectedData.length <= 1 ? '' : ' All'}')),
              const CommandBarSeparator(),
              CommandBarButton(
                  onPressed: () {},
                  icon: DropDownButton(title: const Text("Sort"), items: [
                    for (int i = 0; i < columns.length; i++)
                      MenuFlyoutItem(
                        text: Text(columns[i]),
                        onPressed: () {},
                      )
                  ])),
              CommandBarButton(
                  onPressed: () {},
                  icon: Checkbox(
                    checked: _pr.isSortAscending,
                    onChanged: (value) {
                      setState(() {
                        _pr.isSortAscending = value!;
                      });
                    },
                    style: CheckboxThemeData(
                      checkedIconColor: ButtonState.resolveWith(
                          (states) => FluentTheme.of(context).checkedColor),
                      uncheckedIconColor: ButtonState.resolveWith(
                          (states) => FluentTheme.of(context).checkedColor),
                      checkedDecoration: ButtonState.resolveWith((states) =>
                          BoxDecoration(
                              borderRadius: BorderRadius.circular(5),
                              color: FluentTheme.of(context).accentColor)),
                      uncheckedDecoration: ButtonState.resolveWith((states) =>
                          BoxDecoration(
                              borderRadius: BorderRadius.circular(5),
                              color: FluentTheme.of(context).accentColor)),
                      icon: _pr.isSortAscending
                          ? FluentIcons.up
                          : FluentIcons.down,
                    ),
                  ))
            ],
          ),
        ),
        //
        // Main body
        //
        Expanded(
          child: Container(
            padding:
                const EdgeInsets.only(bottom: 25, left: 50, right: 50, top: 0),
            child: Acrylic(
              elevation: 5,
              blurAmount: 50,
              tint: Colors.white,
              child: Column(
                children: [
                  // Column names
                  ListTile(
                    // Select all button
                    leading: Checkbox(
                      style: const CheckboxThemeData(
                          padding: EdgeInsets.all(0),
                          margin: EdgeInsets.all(0)),
                      checked: _isAllSelected(),
                      onChanged: (value) {
                        setState(
                          () {
                            if (value!) {
                              // can't do _pr.selectedData = filteredData; because
                              // object is not copied
                              for (List<dynamic> row in filteredData) {
                                if (!_pr.selectedData.contains(row)) {
                                  _pr.selectedData.add(row);
                                }
                              }
                            } else {
                              _pr.selectedData = [];
                            }
                          },
                        );
                      },
                    ),
                    title: Row(
                        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                        children: [
                          for (int i = 0; i < columns.length - 1; i++)
                            DatabaseCell(
                              width: MediaQuery.of(context).size.width /
                                  columns.length,
                              text: '${columns[i]}',
                            )
                        ]),
                    trailing: DatabaseCell(
                      text: columns[columns.length - 1],
                      width: 50,
                    ),
                  ),
                  // List of data
                  Expanded(
                    child: DatabaseTable(
                      data: filteredData,
                      editSelected: editSelected,
                    ),
                  ),
                ],
              ),
            ),
          ),
        )
      ],
    );
  }
}
