import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter/services.dart';

Future<String> showDeletePackageDialog(
    BuildContext context, List<List<dynamic>> packages) async {
  final result = await showDialog<String>(
    context: context,
    builder: (context) => ContentDialog(
      title: Text('Delete ${packages.length == 1 ? 'package' : 'packages'}?'),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            'If you delete ${packages.length == 1 ? 'this package' : 'these packages'}, you won\'t be able to recover ${packages.length == 1 ? 'it' : 'them'}. Do you want to delete ${packages.length == 1 ? 'it' : 'them'}?',
          ),
          for (List<dynamic> pack in packages) Text(pack[1])
        ],
      ),
      actions: [
        Button(
          child: const Text('Delete'),
          onPressed: () {
            Navigator.pop(context, 'deleted');
            // Delete file here
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
    BuildContext context, List<List<dynamic>> packages) async {
  final result = await showDialog<String>(
    context: context,
    builder: (context) => ContentDialog(
      title: Text('Update ${packages.length == 1 ? 'package' : 'packages'}?'),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            '${packages.length == 1 ? 'Package' : 'Packages'} currently eligible for update listed below will be updated.',
          ),
          for (List<dynamic> pack in packages)
            Text(pack[1]) // CHECK IF A PACKAGE CAN BE UDPATED FIRST
        ],
      ),
      actions: [
        Button(
          child: const Text('Update'),
          onPressed: () {
            Navigator.pop(context, 'updated');
            // Delete file here
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
  final TextEditingController _controller = TextEditingController();
  final result = await showDialog<String>(
    context: context,
    builder: (context) => ContentDialog(
      title: Text('Add package'),
      content: TextBox(
        placeholder: 'GitHub or npm URL',
        controller: _controller,
      ),
      actions: [
        Button(
          child: const Text('Add'),
          onPressed: () {
            Navigator.pop(context, 'added');
            // Delete file here
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
