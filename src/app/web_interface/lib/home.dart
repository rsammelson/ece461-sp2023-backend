import 'package:fluent_ui/fluent_ui.dart';
import 'package:web_interface/data.dart';

import 'database_table.dart';

class HomePage extends StatefulWidget {
  const HomePage({
    super.key,
  });

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  final TextEditingController _searchController = TextEditingController();
  final PackageRegistry _packageRegistry = PackageRegistry();
  final List<String> columns = ["ID", "Package Name", "New Field", "Status"];
  bool _sortAscending = false;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.center,
      mainAxisAlignment: MainAxisAlignment.start,
      children: [
        Container(
            constraints: const BoxConstraints(maxWidth: 500),
            padding: const EdgeInsets.symmetric(vertical: 10),
            child: AutoSuggestBox(
              controller: _searchController,
              items: const [],
              style: const TextStyle(fontSize: 16),
              clearButtonEnabled: true,
              leadingIcon: const Padding(
                padding: EdgeInsets.all(8),
                child: Icon(FluentIcons.search),
              ),
            )),
        Container(
          padding: const EdgeInsets.symmetric(vertical: 10, horizontal: 50),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              DropDownButton(title: const Text("Sort"), items: [
                for (int i = 0; i < columns.length; i++)
                  MenuFlyoutItem(
                    text: Text(columns[i]),
                    onPressed: () {},
                  )
              ]),
              Checkbox(
                checked: _sortAscending,
                onChanged: (value) {
                  setState(() {
                    _sortAscending = value!;
                  });
                },
                style: CheckboxThemeData(
                  icon: _sortAscending ? FluentIcons.up : FluentIcons.down,
                ),
              )
            ],
          ),
        ),
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
                  Expanded(
                    child: DatabaseTable(
                      data: _packageRegistry.data!,
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
