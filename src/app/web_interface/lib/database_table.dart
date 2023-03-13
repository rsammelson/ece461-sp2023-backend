import 'package:fluent_ui/fluent_ui.dart';

class DatabaseTable extends StatelessWidget {
  DatabaseTable({
    super.key,
    required this.data,
  });
  final List<dynamic> data;
  final List<String> columns = ["id", "Package Name", "Status"];

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 25, left: 50, right: 50, top: 0),
      child: Acrylic(
        elevation: 5,
        blurAmount: 50,
        tint: Colors.white,
        child: ListView.builder(
          scrollDirection: Axis.vertical,
          //shrinkWrap: true,
          itemCount: data.length,
          itemBuilder: (BuildContext context, int index) {
            return DatabaseItem(row: data[index]);
          },
        ),
      ),
    );
  }
}

class DatabaseItem extends StatelessWidget {
  const DatabaseItem({
    super.key,
    required this.row,
  });
  final List<dynamic> row;

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: Container(
        child: Text("${row[0]}"),
        padding: EdgeInsets.symmetric(horizontal: 5),
      ),
      title: Container(
        child: Text("${row[1]}"),
        padding: EdgeInsets.symmetric(horizontal: 5),
      ),
      trailing: Container(
        child: Text("${row[2]}"),
        padding: EdgeInsets.symmetric(horizontal: 5),
      ),
      onPressed: () {},
    );
  }
}
