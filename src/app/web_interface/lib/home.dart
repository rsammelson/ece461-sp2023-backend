import 'package:fluent_ui/fluent_ui.dart';

import 'database_table.dart';

class NavPage extends StatefulWidget {
  const NavPage({super.key, required this.title});

  // Widget for the main page containing the navbar and all other pages within it

  // Title to go on top bar
  final String title;

  @override
  State<NavPage> createState() => _NavPageState();
}

class _NavPageState extends State<NavPage> {
  // Index of current page
  int _pageIndex = 0;
  final GlobalKey _viewKey = GlobalKey();

  @override
  Widget build(BuildContext context) {
    return NavigationView(
      key: _viewKey,
      appBar: const NavigationAppBar(
        automaticallyImplyLeading: false,
        title: Text("ACMEIR Package Registry", style: TextStyle(fontSize: 18)),
      ),
      pane: NavigationPane(
          selected: _pageIndex,
          items: [
            // Home page navbar item
            PaneItem(
                icon: const Icon(FluentIcons.a_a_d_logo),
                title: const Text(
                  "Home",
                ),
                body: const HomePage())
          ],
          onChanged: (value) {
            _pageIndex = value;
          },
          displayMode: PaneDisplayMode.compact),
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({
    super.key,
  });

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  final TextEditingController _searchController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.center,
      mainAxisAlignment: MainAxisAlignment.start,
      children: [
        Container(
            constraints: const BoxConstraints(maxWidth: 500),
            padding: EdgeInsets.symmetric(vertical: 10),
            child: AutoSuggestBox(
              controller: _searchController,
              items: [],
              style: TextStyle(fontSize: 16),
              clearButtonEnabled: true,
              leadingIcon: Padding(
                padding: EdgeInsets.all(8),
                child: Icon(FluentIcons.search),
              ),
            )),
        Container(
          padding: EdgeInsets.symmetric(vertical: 10, horizontal: 50),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              DropDownButton(title: Text("Sort"), items: [
                MenuFlyoutItem(
                  text: Text("id"),
                  onPressed: () {},
                ),
                MenuFlyoutItem(
                  text: Text("Name"),
                  onPressed: () {},
                ),
                MenuFlyoutItem(
                  text: Text("Status"),
                  onPressed: () {},
                ),
              ]),
              Checkbox(
                  checked: true,
                  onChanged: (value) {},
                  style: CheckboxThemeData(
                      uncheckedIconColor:
                          CheckboxTheme.of(context).checkedIconColor,
                      icon: FluentIcons.up))
            ],
          ),
        ),
        Expanded(
          child: DatabaseTable(
            data: [
              ["1", "package name", "OK"],
              ["2", "package name", "OK"]
            ],
          ),
        )
      ],
    );
  }
}
