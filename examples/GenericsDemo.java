package examples;

import java.util.ArrayList;
import java.util.List;

public class GenericsDemo {
    public void demo() {
        List<String> list = new ArrayList<>();
        list.add("Hello");
        list.add("World");
        for (String item : list) {
            System.out.println(item);
        }
    }
}