package examples;

import java.util.function.Function;

public class LambdaDemo {
    public void demo() {
        Function<String, Integer> lengthFunction = s -> s.length();
        int length = lengthFunction.apply("Hello, World!");
        System.out.println(length);
    }
}