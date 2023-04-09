import 'package:fluent_ui/fluent_ui.dart';

import 'data.dart' show PackageRegistry;
import 'main.dart' show trailingSize;
import 'popup.dart' show showPropertiesDialog;

class DatabaseTable extends StatelessWidget {
  const DatabaseTable({
    super.key,
    required this.data,
    required this.editSelected,
  });
  final List<Map<String, dynamic>> data;
  final Function editSelected;

  @override
  Widget build(BuildContext context) {
    // ListView with ListTiles as items, built using input data
    return ListView.builder(
      scrollDirection: Axis.vertical,
      shrinkWrap: false,
      itemCount: data.length,
      itemBuilder: (BuildContext context, int index) {
        return DatabaseRow(
          editSelected: editSelected,
          cells: data[index],
        );
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
    required this.editSelected,
  });
  final Map<String, dynamic> cells;
  final Function editSelected;

  @override
  Widget build(BuildContext context) {
    return ListTile.selectable(
      selectionMode: ListTileSelectionMode.multiple,
      onSelectionChange: (value) {
        editSelected(value, cells);
      },
      selected: PackageRegistry().selectedData.contains(cells),
      title: Row(mainAxisAlignment: MainAxisAlignment.spaceEvenly, children: [
        DatabaseCell(
          width: MediaQuery.of(context).size.width / (cells.length),
          text: '${cells['id']}',
        ),
        DatabaseCell(
          width: MediaQuery.of(context).size.width / (cells.length),
          text: '${cells['name']}',
        ),
        DatabaseCell(
          width: MediaQuery.of(context).size.width / (cells.length),
          text: '${cells['version']}',
        ),
        DatabaseCell(
          width: MediaQuery.of(context).size.width / (cells.length),
          text: double.parse('${cells['rating']}').toStringAsFixed(2),
        ),
      ]),
      trailing: SizedBox(
        width: trailingSize,
        child: FilledButton(
          onPressed: () async {
            await showPropertiesDialog(context, data: cells);
          },
          child: const Text("Properties"),
        ),
      ),
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
        text ?? '',
        semanticsLabel: text ?? '',
        textAlign: TextAlign.center,
      ),
    );
  }
}
