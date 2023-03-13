import 'package:fluent_ui/fluent_ui.dart';

import 'package:web_interface/home.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
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
      title: 'ACMEIR Package Registry',
      themeMode: _themeMode,
      theme: ThemeData(brightness: Brightness.light),
      darkTheme: ThemeData(brightness: Brightness.dark),
      home: const HomePage(title: 'ACMEIR Package Registry'),
    );
  }
}
