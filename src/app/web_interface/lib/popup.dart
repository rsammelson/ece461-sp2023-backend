import 'package:fluent_ui/fluent_ui.dart';

Future<String> showDeletePackageDialog(
    BuildContext context, List<Map<String, dynamic>> packages) async {
  final result = await showDialog<String>(
    context: context,
    builder: (context) => ContentDialog(
      title: Text('Delete ${packages.length == 1 ? 'package' : 'packages'}?'),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            'If you delete ${packages.length == 1 ? 'this package' : 'these packages'}, you won\'t be able to recover ${packages.length == 1 ? 'it' : 'them'}. Do you want to delete ${packages.length == 1 ? 'it' : 'them'}?',
            style: const TextStyle(fontSize: 16),
          ),
          for (Map<String, dynamic> pack in packages) Text(pack['name'])
        ],
      ),
      actions: [
        Button(
          child: const Text('Delete'),
          onPressed: () {
            Navigator.pop(context, 'deleted');
            // Delete packages here
          },
        ),
        FilledButton(
          child: const Text('Cancel'),
          onPressed: () => Navigator.pop(context, 'canceled'),
        ),
      ],
    ),
  );
  return result ?? 'canceled';
}

Future<String> showUpdatePackageDialog(
    BuildContext context, List<Map<String, dynamic>> packages) async {
  final result = await showDialog<String>(
    context: context,
    builder: (context) => ContentDialog(
      title: Text('Update ${packages.length == 1 ? 'package' : 'packages'}?'),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            '${packages.length == 1 ? 'Package' : 'Packages'} currently eligible for update listed below will be updated.',
            style: const TextStyle(fontSize: 16),
          ),
          for (Map<String, dynamic> pack in packages)
            Text(pack['name']) // CHECK IF A PACKAGE CAN BE UDPATED FIRST
        ],
      ),
      actions: [
        Button(
          child: const Text('Update'),
          onPressed: () {
            Navigator.pop(context, 'updated');
            // Update packages here
          },
        ),
        FilledButton(
          child: const Text('Cancel'),
          onPressed: () => Navigator.pop(context, 'canceled'),
        ),
      ],
    ),
  );
  return result ?? 'canceled';
}

Future<String> showAddPackageDialog(BuildContext context) async {
  final TextEditingController controller = TextEditingController();
  final result = await showDialog<String>(
    context: context,
    builder: (context) => ContentDialog(
      title: const Text('Add package'),
      content: TextBox(
        placeholder: 'GitHub or npm URL',
        controller: controller,
      ),
      actions: [
        Button(
          child: const Text('Add'),
          onPressed: () {
            Navigator.pop(context, controller.text);
            // Add package here
            // Must check if package with same name and version already exists or not
          },
        ),
        FilledButton(
          child: const Text('Cancel'),
          onPressed: () => Navigator.pop(context, 'canceled'),
        ),
      ],
    ),
  );
  return result ?? 'canceled';
}

Future<String> showPropertiesDialog(BuildContext context,
    {required Map<String, dynamic> data}) async {
  final result = await showDialog<String>(
    context: context,
    builder: (context) => ContentDialog(
      style: const ContentDialogThemeData(bodyStyle: TextStyle(fontSize: 20)),
      title: const Text('Properties'),
      content: SingleChildScrollView(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            propertyRow(name: 'Name', value: data['name'].toString()),
            propertyRow(name: 'ID', value: data['id'].toString()),
            propertyRow(
                name: 'Rating',
                value: double.parse('${data['rating']}').toStringAsFixed(2)),
            propertyRow(name: 'Version', value: data['version'].toString()),
            propertyRow(name: 'Description', value: data['info'].toString()),
            propertyRow(name: 'URL', value: data['url'].toString()),
          ],
        ),
      ),
      actions: [
        FilledButton(
          child: const Text('Close'),
          onPressed: () => Navigator.pop(context, 'canceled'),
        ),
      ],
    ),
  );
  return result ?? 'canceled';
}

Widget propertyRow({required String name, required String value}) {
  return Container(
    decoration: BoxDecoration(
        color: const Color.fromARGB(255, 235, 235, 235),
        borderRadius: BorderRadius.circular(7)),
    padding: const EdgeInsets.symmetric(vertical: 10, horizontal: 10),
    margin: const EdgeInsets.all(5),
    child: Column(
      children: [
        Padding(
          padding: EdgeInsets.only(bottom: (value.length > 20) ? 7 : 0),
          child: Row(
            children: [
              Text(
                style: const TextStyle(fontWeight: FontWeight.bold),
                name,
                textAlign: TextAlign.start,
              ),
              const Spacer(),
              Text(
                value.length > 20 ? '' : value,
                textAlign: TextAlign.end,
              )
            ],
          ),
        ),
        if (value.length > 20)
          GestureDetector(
            onTap: () {
              if (name == 'URL') {
                // TODO
              }
            },
            child: Text(
              style: TextStyle(
                  fontSize: 14,
                  color: (name == 'URL') ? Colors.blue : Colors.black,
                  decoration: (name == 'URL')
                      ? TextDecoration.underline
                      : TextDecoration.none),
              overflow: TextOverflow.fade,
              softWrap: true,
              maxLines: 6,
              value,
            ),
          )
      ],
    ),
  );
}
