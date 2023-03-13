import 'package:fluent_ui/fluent_ui.dart';

class DatabaseTable extends StatelessWidget {
  const DatabaseTable({
    super.key,
    required this.data,
  });
  final List<List<dynamic>> data;
  // final List<String> columns = ["id", "Package Name", "Status"];

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      scrollDirection: Axis.vertical,
      //shrinkWrap: true,
      itemCount: data.length,
      itemBuilder: (BuildContext context, int index) {
        return DatabaseItem(row: data[index]);
      },
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
      leading: DatabaseCell(text: row[0]),
      title: Row(mainAxisAlignment: MainAxisAlignment.spaceEvenly, children: [
        for (int i = 1; i < row.length - 1; i++)
          DatabaseCell(
            width: MediaQuery.of(context).size.width / row.length,
            text: row[i],
          )
      ]),
      trailing: DatabaseCell(text: row[row.length - 1]),
      onPressed: () {},
    );
  }
}

class DatabaseCell extends StatelessWidget {
  const DatabaseCell({
    super.key,
    this.text,
    this.width,
  });

  final String? text;
  final double? width;

  @override
  Widget build(BuildContext context) {
    return Container(
      width: width,
      padding: const EdgeInsets.symmetric(horizontal: 5),
      child: Text(text ?? ""),
    );
  }
}
