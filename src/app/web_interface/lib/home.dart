import 'package:fluent_ui/fluent_ui.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key, required this.title});

  final String title;

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  int _pageIndex = 0;
  final GlobalKey viewKey = GlobalKey();

  @override
  Widget build(BuildContext context) {
    return Text("");
  }
}
