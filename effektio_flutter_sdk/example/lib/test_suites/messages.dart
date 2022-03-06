import 'package:effektio_flutter_sdk/effektio_flutter_sdk.dart';
import 'package:effektio_flutter_sdk_example/constants.dart';
import 'package:effektio_flutter_sdk_example/test_suites/interface.dart';

class MessagesTest extends TestSuite {
  @override
  Stream<String> executeTest() async* {
    yield 'Initializing SDK';
    final sdk = await EffektioSdk.instance;
    yield 'Logging in';
    final client = await sdk.login(username, password);

    while (true) {
      yield 'waiting for sync';
      if (client.hasFirstSynced()) {
        break;
      } else {
        await Future.delayed(const Duration(seconds: 10));
      }
    }

    yield 'Fetching rooms';
    final rooms = client.conversations();
    final roomNames = await Future.wait(rooms.map((r) => r.displayName()));
    yield 'Rooms: ${roomNames.toString()}';
    final testRoomIndex = roomNames.indexWhere((element) => element.toLowerCase().contains('sagar'));
    final testRoom = rooms[testRoomIndex];
    yield await testRoom.message();
    yield 'done';
  }

  @override
  Future<void> setup() async {}

  @override
  Future<void> teardown() async {}
}
