import 'package:fluent_ui/fluent_ui.dart';

import 'main.dart' show offwhite;

class LoginPage extends StatefulWidget {
  const LoginPage({
    super.key,
  });

  @override
  State<LoginPage> createState() => _LoginPageState();
}

class _LoginPageState extends State<LoginPage> {
  bool showingPass = false;
  bool invalidPass = true;
  bool invalidUser = true;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Container(
        constraints: const BoxConstraints(
            maxHeight: 500, maxWidth: 400, minHeight: 100, minWidth: 100),
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(25),
          color: Colors.white,
        ),
        child: Column(mainAxisAlignment: MainAxisAlignment.center, children: [
          Padding(
            padding: const EdgeInsets.only(bottom: 20),
            child: Text(
              "Login",
              style: TextStyle(fontSize: 32),
            ),
          ),
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 15, vertical: 5),
            child: LoginTextBox(
              invalidText: 'Incorrect Username',
              // showInvalidText: invalidUser,
              invalid: invalidUser,
              hintText: 'Username',
            ),
          ),
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 15, vertical: 5),
            child: LoginTextBox(
              invalidText: 'Incorrect Password',
              // showInvalidText: invalidPass,
              invalid: invalidPass,
              showPass: showingPass,
              showPassClick: () {
                setState(() {
                  showingPass = !showingPass;
                });
              },
              isPassword: true,
              hintText: 'Password',
            ),
          ),
          GestureDetector(
            onTap: () {
              setState(() {
                invalidPass = !invalidPass;
                invalidUser = !invalidUser;
              });
            },
            child: Container(
              padding: const EdgeInsets.symmetric(vertical: 10, horizontal: 40),
              margin: const EdgeInsets.all(15),
              decoration: BoxDecoration(
                borderRadius: BorderRadius.circular(25),
                color: Color.fromARGB(255, 175, 134, 0),
              ),
              child: const Text(
                'Sign in',
                style: TextStyle(color: Colors.white, fontSize: 20),
              ),
            ),
          )
        ]),
      ),
    );
  }
}

class LoginTextBox extends StatelessWidget {
  const LoginTextBox(
      {super.key,
      this.hintText,
      this.invalidText,
      this.textFieldController,
      this.isPassword = false,
      this.showInvalidText = false,
      this.invalid = false,
      this.show = true,
      this.opacity = 1,
      this.showPass = false,
      this.showPassClick});
  final String? hintText;
  final String? invalidText;
  final TextEditingController? textFieldController;
  final bool isPassword;
  final bool invalid;
  final bool showInvalidText;
  final bool show;
  final bool showPass;
  final double opacity;
  final void Function()? showPassClick;

  @override
  Widget build(BuildContext context) {
    return AnimatedContainer(
      height: show ? (showInvalidText ? 76 : 52) : 0,
      duration: const Duration(milliseconds: 150),
      child: SingleChildScrollView(
        physics: const NeverScrollableScrollPhysics(),
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 250),
          padding: const EdgeInsets.symmetric(vertical: 5, horizontal: 5),
          decoration: BoxDecoration(
              color: offwhite,
              border: Border.all(
                  color: showInvalidText || invalid
                      ? Colors.red
                      : offwhite, //Theme.of(context).colorScheme.secondary,
                  width: 2),
              borderRadius: BorderRadius.circular(12)),
          constraints: const BoxConstraints(maxWidth: 500),
          width: MediaQuery.of(context).size.width / 1.5,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              if (showInvalidText)
                Padding(
                  padding: const EdgeInsets.only(top: 5),
                  child: Text(
                    invalidText ?? "Incorrect $hintText",
                    style: TextStyle(
                        color: Colors.red, fontWeight: FontWeight.bold),
                  ),
                ),
              TextBox(
                suffixMode: isPassword
                    ? OverlayVisibilityMode.editing
                    : OverlayVisibilityMode.never,
                suffix: IconButton(
                  icon: Icon(
                    showPass
                        ? FluentIcons.visually_impaired
                        : FluentIcons.red_eye12,
                    size: 26,
                  ),
                  onPressed: showPassClick,
                ),
                style: TextStyle(fontSize: 18),
                foregroundDecoration: BoxDecoration(
                    color: Colors.transparent,
                    border: Border.all(color: Colors.transparent)),
                placeholder: hintText,
                controller: textFieldController,
                obscureText: isPassword && !showPass,
                cursorColor: showInvalidText ? offwhite : Colors.black,
                decoration: BoxDecoration(
                    color: Colors.transparent,
                    border: Border.all(color: Colors.transparent)),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
