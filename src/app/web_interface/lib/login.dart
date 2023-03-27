import 'package:firebase_auth/firebase_auth.dart';
import 'package:fluent_ui/fluent_ui.dart';

import 'main.dart' show offwhite, offwhiteDark;
import 'wavy_bg.dart' show WavingBackground;

class LoginPage extends StatefulWidget {
  const LoginPage({
    super.key,
  });

  @override
  State<LoginPage> createState() => _LoginPageState();
}

class _LoginPageState extends State<LoginPage> {
  final TextEditingController _userController = TextEditingController();
  final TextEditingController _passController = TextEditingController();
  bool showingPass = false;
  bool invalidPass = false;
  bool invalidUser = false;
  bool tooManyAttempts = false;
  bool isWorking = false;

  signInBtnPress() async {
    // Turn on progress indicator
    setState(() {
      isWorking = true;
    });
    invalidPass = _passController.text == '' ? true : false;
    invalidUser = _userController.text == '' ? true : false;
    // Possible error codes:
    // wrong-password
    // user-not-found
    // invalid-email
    // too-many-requests

    // Try to sign in user, or get error response
    if (!invalidPass && !invalidUser) {
      try {
        await FirebaseAuth.instance.signInWithEmailAndPassword(
            email: '${_userController.text}@acme.project461',
            password: _passController.text);
        _userController.clear();
        _passController.clear();
      } on FirebaseAuthException catch (e) {
        if (e.code.contains("password")) {
          invalidPass = true;
        } else if (e.code.contains("not-found")) {
          invalidUser = true;
        } else if (e.code.contains("too-many")) {
          tooManyAttempts = true;
        } else if (e.code.contains("email")) {
          invalidUser = true;
        }
      }
    }

    // Reset button and show logged in
    setState(() {
      isWorking = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    return WavingBackground(
      child: Center(
        child: Container(
          constraints: const BoxConstraints(
              maxHeight: 500, maxWidth: 400, minHeight: 100, minWidth: 100),
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(25),
            color: offwhite,
          ),
          child: FirebaseAuth.instance.currentUser == null
              ? _buildLogin()
              : _buildLogout(),
        ),
      ),
    );
  }

  Column _buildLogout() {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Padding(
          padding: EdgeInsets.only(bottom: 20, left: 50, right: 50),
          child: Text(
            "Hello ${FirebaseAuth.instance.currentUser?.email?.split('@')[0]}!",
            overflow: TextOverflow.fade,
            semanticsLabel:
                "Hello ${FirebaseAuth.instance.currentUser?.email?.split('@')[0]}!",
            softWrap: true,
            maxLines: 3,
            style: TextStyle(fontSize: 28),
          ),
        ),
        Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            GestureDetector(
              onTap: () async {
                // Wait till process complete
                setState(() {
                  isWorking = true;
                });
                await FirebaseAuth.instance.signOut();

                setState(() {
                  isWorking = false;
                });
              },
              child: AnimatedContainer(
                width: isWorking ? 104 : 176,
                duration: const Duration(milliseconds: 150),
                padding:
                    const EdgeInsets.symmetric(vertical: 10, horizontal: 40),
                margin: const EdgeInsets.all(15),
                decoration: BoxDecoration(
                  borderRadius: BorderRadius.circular(25),
                  color: isWorking ? Colors.white : Colors.blue,
                ),
                child: isWorking
                    ? Row(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: const [
                          SizedBox(
                              width: 24, height: 24, child: ProgressRing()),
                        ],
                      )
                    : Row(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: const [
                          Padding(
                            padding: EdgeInsets.only(right: 15),
                            child: Icon(
                              FluentIcons.sign_out,
                              color: Colors.white,
                            ),
                          ),
                          Text(
                            semanticsLabel: 'Logout',
                            textAlign: TextAlign.center,
                            softWrap: true,
                            overflow: TextOverflow.fade,
                            'Logout',
                            style: TextStyle(color: Colors.white, fontSize: 20),
                          ),
                        ],
                      ),
              ),
            ),
          ],
        ),
      ],
    );
  }

  Column _buildLogin() {
    return Column(mainAxisAlignment: MainAxisAlignment.center, children: [
      const Padding(
        padding: EdgeInsets.only(bottom: 20),
        child: Text(
          "Login",
          style: TextStyle(fontSize: 32),
        ),
      ),
      Container(
        padding: const EdgeInsets.symmetric(horizontal: 15, vertical: 5),
        child: LoginTextBox(
          textFieldController: _userController,
          invalidText: 'Incorrect Username',
          // showInvalidText: invalidUser,
          invalid: invalidUser,
          hintText: 'Username',
        ),
      ),
      Container(
        padding: const EdgeInsets.symmetric(horizontal: 15, vertical: 5),
        child: LoginTextBox(
          textFieldController: _passController,
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
      if (tooManyAttempts)
        Container(
          padding: EdgeInsets.symmetric(horizontal: 10, vertical: 15),
          child: Text(
            "Too many attempts, try again later",
            style: TextStyle(color: Colors.red),
          ),
          decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(20), color: offwhite),
        ),
      Container(
        height: 75,
        child: GestureDetector(
          onTap: () async {
            // Wait till process complete
            await signInBtnPress();
          },
          child: AnimatedContainer(
            width: isWorking ? 104 : 140,
            height: 44,
            duration: const Duration(milliseconds: 150),
            padding: const EdgeInsets.symmetric(vertical: 10, horizontal: 40),
            margin: const EdgeInsets.all(15),
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(25),
              color: isWorking ? Colors.white : Colors.blue,
            ),
            child: isWorking
                ? Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: const [
                      SizedBox(width: 24, height: 24, child: ProgressRing()),
                    ],
                  )
                : Text(
                    textAlign: TextAlign.center,
                    overflow: TextOverflow.fade,
                    maxLines: 1,
                    'Sign in',
                    style: TextStyle(color: Colors.white, fontSize: 20),
                  ),
          ),
        ),
      )
    ]);
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
              color: offwhiteDark,
              border: Border.all(
                  color: showInvalidText || invalid ? Colors.red : offwhiteDark,
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
                style: const TextStyle(fontSize: 18),
                foregroundDecoration: BoxDecoration(
                    color: Colors.transparent,
                    border: Border.all(color: Colors.transparent)),
                placeholder: hintText,
                controller: textFieldController,
                obscureText: isPassword && !showPass,
                cursorColor: showInvalidText ? offwhiteDark : Colors.black,
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
