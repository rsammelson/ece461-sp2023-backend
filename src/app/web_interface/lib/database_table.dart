import 'package:fluent_ui/fluent_ui.dart';

class DatabaseTable extends StatelessWidget {
  const DatabaseTable({
    super.key,
    required this.data,
  });
  final List<List<dynamic>> data;

  @override
  Widget build(BuildContext context) {
    // ListView with ListTiles as items, built using input data
    return ListView.builder(
      scrollDirection: Axis.vertical,
      shrinkWrap: false,
      itemCount: data.length,
      itemBuilder: (BuildContext context, int index) {
        return DatabaseRow(cells: data[index]);
      },
    );
  }
}

class DatabaseRow extends StatelessWidget {
  // A row in the DatabaseTable
  // The first item in the list is used as leading in ListTile
  // The last item in the list is used as trailing in ListTile
  // All others items in the list are spread out evenly in the title of ListTile
  const DatabaseRow({
    super.key,
    required this.cells,
  });
  final List<dynamic> cells;

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: DatabaseCell(text: cells[0]),
      title: Row(mainAxisAlignment: MainAxisAlignment.spaceEvenly, children: [
        for (int i = 1; i < cells.length - 1; i++)
          DatabaseCell(
            width: MediaQuery.of(context).size.width / cells.length,
            text: '${cells[i]}',
          )
      ]),
      trailing: DatabaseCell(text: cells[cells.length - 1]),
      onPressed: () {},
    );
  }
}

class DatabaseCell extends StatelessWidget {
  // A single cell in the row (DatabaseRow)
  const DatabaseCell({
    super.key,
    this.text,
    this.width,
    this.hpad = 5,
  });

  final String? text;
  final double? width;
  final double? hpad;

  @override
  Widget build(BuildContext context) {
    return Container(
      width: width,
      padding: EdgeInsets.symmetric(horizontal: hpad!),
      child: Text(
        text ?? "",
        textAlign: TextAlign.center,
      ),
    );
  }
}
