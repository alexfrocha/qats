import { Link } from "expo-router";
import { Text, View } from "react-native";

export default function RootPage() {
  return (
    <View>
      <Text className="text-red-500">
        hey man, how u doing, in root page rn
      </Text>
      <Link href={"/teste"}>go to index page</Link>
    </View>
  );
}
