import 'dart:math' show cos, pi, sin;
import 'package:fluent_ui/fluent_ui.dart';

class DrawClip extends CustomClipper<Path> {
  double move;
  DrawClip({this.move = 0});

  @override
  Path getClip(Size size) {
    Path path = Path();

    const double hFactor = 0.15;

    // x and y location for first point (bottom left)
    path.lineTo(0, size.height);
    // x and y location for second point (bottom right)
    path.lineTo(size.width, size.height);
    // x and y location for third point (top right)
    path.lineTo(size.width, size.height * hFactor);

    // calculate x and y location for final point (top left)
    double xCenter = size.width * 0.5 + (size.width * 0.6 + 1) * sin(move * pi);
    double yCenter = size.height * hFactor + 50 * cos(move * pi);

    // x and y location for last points
    path.quadraticBezierTo(xCenter, yCenter, 0, size.height * hFactor);

    return path;
  }

  @override
  bool shouldReclip(CustomClipper<Path> oldClipper) {
    return true;
  }
}

class WavingBackground extends StatefulWidget {
  const WavingBackground({Key? key, required this.child, this.clipper})
      : super(key: key);

  final Widget child;
  final CustomClipper<Path>? clipper;
  @override
  State<WavingBackground> createState() => _WavingBackgroundState();
}

class _WavingBackgroundState extends State<WavingBackground>
    with TickerProviderStateMixin {
  late final AnimationController _animationController = AnimationController(
      vsync: this,
      value: 0.0,
      lowerBound: -1.0,
      duration: const Duration(milliseconds: 5000))
    ..repeat();
  @override
  void dispose() {
    _animationController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Align(
          alignment: Alignment.bottomCenter,
          child: AnimatedBuilder(
            animation: _animationController,
            builder: (BuildContext context, Widget? child) {
              return ClipPath(
                clipper: DrawClip(move: _animationController.value),
                child: Container(
                  height: MediaQuery.of(context).size.height * 0.7,
                  decoration: BoxDecoration(
                    gradient: LinearGradient(
                        begin: Alignment.topCenter,
                        end: Alignment.bottomCenter,
                        colors: [
                          Colors.blue.withOpacity(0.25),
                          Colors.blue.withOpacity(0.75)
                        ]),
                  ),
                ),
              );
            },
          ),
        ),
        widget.child,
      ],
    );
  }
}
