import { Link } from "expo-router";
import { Text, View } from "react-native";

export default function IndexPage() {
  return (
    <View>
      <Text>now i'm on index page</Text>
      <Link href={"/"}>go to root page</Link>
    </View>
  );
}
