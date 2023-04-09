import 'package:firebase_auth/firebase_auth.dart';
import 'package:firebase_core/firebase_core.dart';
import 'package:fluent_ui/fluent_ui.dart';

import 'data.dart' show PackageRegistry;
import 'firebase_options.dart' show DefaultFirebaseOptions;
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
const Color offwhite = Color.fromARGB(255, 241, 241, 241);
const Color offwhiteDark = Color.fromARGB(255, 222, 222, 222);

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // setup firebase
  await Firebase.initializeApp(
    options: DefaultFirebaseOptions.currentPlatform,
  );

  runApp(const WebApp());
}

class WebApp extends StatefulWidget {
  const WebApp({super.key});

  @override
  State<WebApp> createState() => _WebAppState();
}

class _WebAppState extends State<WebApp> with WidgetsBindingObserver {
  // Theme
  final ThemeMode _themeMode = ThemeMode.system;
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    super.dispose();
  }

  @override
  Future<void> didChangeAppLifecycleState(AppLifecycleState state) async {
    super.didChangeAppLifecycleState(state);
    if (state == AppLifecycleState.detached) {
      FirebaseAuth.instance.signOut();
    }
  }

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
      transitionBuilder: (child, animation) {
        return HorizontalSlidePageTransition(
            animation: animation, child: child);
      },
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
                body: FutureBuilder(
                  builder: (context, snapshot) {
                    return HomePage(importDataSuccess: snapshot.data ?? false);
                  },
                  future: PackageRegistry().importData(),
                )),
            // Login navbar item
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
          displayMode: PaneDisplayMode.minimal),
    );
  }
}
