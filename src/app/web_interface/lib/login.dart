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
  bool resetInProgress = false;

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
    // permission-denied

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
          padding: const EdgeInsets.only(bottom: 20, left: 50, right: 50),
          child: Text(
            "Hello ${FirebaseAuth.instance.currentUser?.email?.split('@')[0]}!",
            overflow: TextOverflow.fade,
            semanticsLabel:
                "Hello ${FirebaseAuth.instance.currentUser?.email?.split('@')[0]}!",
            softWrap: true,
            maxLines: 3,
            style: const TextStyle(fontSize: 28),
          ),
        ),
        Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            // Reset app
            GestureDetector(
              onTap: () async {
                // Wait till process complete
                if (!isWorking) {
                  setState(() {
                    resetInProgress = true;
                  });
                  // RESET
                  await Future.delayed(Duration(seconds: 2));

                  setState(() {
                    resetInProgress = false;
                  });
                }
              },
              child: AnimatedContainer(
                width: resetInProgress ? 104 : 220,
                height: 44,
                duration: const Duration(milliseconds: 150),
                padding:
                    const EdgeInsets.symmetric(vertical: 10, horizontal: 40),
                margin: const EdgeInsets.all(15),
                decoration: BoxDecoration(
                  borderRadius: BorderRadius.circular(25),
                  color: isWorking
                      ? Color.fromARGB(255, 94, 94, 94)
                      : (resetInProgress ? Colors.white : Colors.red),
                ),
                child: resetInProgress
                    ? Row(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: const [
                          SizedBox(
                              width: 24, height: 24, child: ProgressRing()),
                        ],
                      )
                    : const Text(
                        semanticsLabel: 'Factory reset',
                        maxLines: 1,
                        textAlign: TextAlign.center,
                        'Factory reset',
                        style: TextStyle(color: Colors.white, fontSize: 20),
                      ),
              ),
            ),
            // Logout
            GestureDetector(
              onTap: () async {
                // Wait till process complete
                if (!resetInProgress) {
                  setState(() {
                    isWorking = true;
                  });
                  await FirebaseAuth.instance.signOut();

                  setState(() {
                    isWorking = false;
                  });
                }
              },
              child: AnimatedContainer(
                width: isWorking ? 104 : 176,
                height: 44,
                duration: const Duration(milliseconds: 150),
                padding:
                    const EdgeInsets.symmetric(vertical: 10, horizontal: 40),
                margin: const EdgeInsets.all(15),
                decoration: BoxDecoration(
                  borderRadius: BorderRadius.circular(25),
                  color: resetInProgress
                      ? Color.fromARGB(255, 94, 94, 94)
                      : (isWorking ? Colors.white : Colors.blue),
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
                            maxLines: 1,
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
          invalid: invalidUser,
          hintText: 'Username',
        ),
      ),
      Container(
        padding: const EdgeInsets.symmetric(horizontal: 15, vertical: 5),
        child: LoginTextBox(
          textFieldController: _passController,
          invalidText: 'Incorrect Password',
          invalid: invalidPass,
          showPass: showingPass,
          showPassClick: () {
            setState(() {
              showingPass = !showingPass;
            });
          },
          isPassword: true,
          hintText: 'Password',
          onSubmit: (String? value) async {
            await signInBtnPress();
          },
        ),
      ),
      if (tooManyAttempts)
        Container(
          padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 15),
          decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(20), color: offwhite),
          child: Text(
            "Too many attempts, try again later",
            style: TextStyle(color: Colors.red),
          ),
        ),
      SizedBox(
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
                : const Text(
                    semanticsLabel: 'Sign in',
                    textAlign: TextAlign.center,
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
      this.invalid = false,
      this.showPass = false,
      this.showPassClick,
      this.onSubmit});
  final String? hintText;
  final String? invalidText;
  final TextEditingController? textFieldController;
  final bool isPassword;
  final bool invalid;
  final bool showPass;
  final void Function()? showPassClick;
  final void Function(String?)? onSubmit;

  @override
  Widget build(BuildContext context) {
    return AnimatedContainer(
      duration: const Duration(milliseconds: 250),
      padding: const EdgeInsets.symmetric(vertical: 5, horizontal: 5),
      decoration: BoxDecoration(
          color: offwhiteDark,
          border:
              Border.all(color: invalid ? Colors.red : offwhiteDark, width: 2),
          borderRadius: BorderRadius.circular(12)),
      constraints: const BoxConstraints(maxWidth: 500),
      width: MediaQuery.of(context).size.width / 1.5,
      child: TextBox(
        onSubmitted: onSubmit,
        suffixMode: isPassword
            ? OverlayVisibilityMode.editing
            : OverlayVisibilityMode.never,
        suffix: IconButton(
          icon: Icon(
            showPass ? FluentIcons.visually_impaired : FluentIcons.red_eye12,
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
        decoration: BoxDecoration(
            color: Colors.transparent,
            border: Border.all(color: Colors.transparent)),
      ),
    );
  }
}
