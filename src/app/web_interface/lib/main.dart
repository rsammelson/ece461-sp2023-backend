import 'package:fluent_ui/fluent_ui.dart';

import 'data.dart' show PackageRegistry;
import 'home.dart' show HomePage;
import 'login.dart' show LoginPage;

//
// Constants

const List<String> columns = [
  "ID",
  "Package Name",
  "Version",
  "Rating",
  "Properties"
];
const double trailingSize = 100.0;
const String siteName = 'ACME Package Registry';
const Color offwhite = Color.fromARGB(255, 222, 222, 222);
// Do we have IDs for each package?
// Use Firebase for auth?
// What are the properties

void main() {
  WidgetsFlutterBinding.ensureInitialized();

  // set up data registry
  PackageRegistry().importData();

  runApp(const WebApp());
}

class WebApp extends StatelessWidget {
  const WebApp({super.key});
  // Theme
  final ThemeMode _themeMode = ThemeMode.system;

  @override
  Widget build(BuildContext context) {
    return FluentApp(
      debugShowCheckedModeBanner: false,
      title: siteName,
      themeMode: _themeMode,
      theme: FluentThemeData(brightness: Brightness.light),
      darkTheme: FluentThemeData(brightness: Brightness.dark),
      home: const NavPage(title: siteName),
    );
  }
}

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
        title: Text(siteName, style: TextStyle(fontSize: 18)),
      ),
      pane: NavigationPane(
          selected: _pageIndex,
          items: [
            // Home page navbar item
            PaneItem(
                icon: const Icon(FluentIcons.home),
                title: const Text(
                  "Home",
                ),
                body: const HomePage()),
            PaneItem(
                icon: const Icon(FluentIcons.contact),
                title: const Text(
                  "Login",
                ),
                body: const LoginPage()),
          ],
          onChanged: (value) {
            setState(() {
              _pageIndex = value;
            });
          },
          displayMode: PaneDisplayMode.compact),
    );
  }
}
