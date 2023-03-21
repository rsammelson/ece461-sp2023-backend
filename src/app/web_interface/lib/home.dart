import 'package:fluent_ui/fluent_ui.dart';
import 'package:web_interface/data.dart';

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

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.center,
      mainAxisAlignment: MainAxisAlignment.start,
      children: [
        // Search bar
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
        // Filter options
        Container(
          padding: const EdgeInsets.symmetric(vertical: 10, horizontal: 50),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              // Sort drop down
              DropDownButton(title: const Text("Sort"), items: [
                for (int i = 0; i < columns.length; i++)
                  MenuFlyoutItem(
                    text: Text(columns[i]),
                    onPressed: () {},
                  )
              ]),
              // Ascending or descending sort checkbox
              Checkbox(
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
                  icon: _pr.isSortAscending ? FluentIcons.up : FluentIcons.down,
                ),
              )
            ],
          ),
        ),
        // Main body
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
                    leading: DatabaseCell(text: columns[0]),
                    title: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                      children: [
                        for (int i = 1; i < columns.length - 1; i++)
                          DatabaseCell(
                            width: MediaQuery.of(context).size.width /
                                columns.length,
                            text: columns[i],
                          )
                      ],
                    ),
                    trailing: DatabaseCell(text: columns[columns.length - 1]),
                  ),
                  // List of data
                  Expanded(
                    child: DatabaseTable(
                      data: filteredData,
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
