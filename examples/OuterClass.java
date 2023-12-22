package examples;

public class OuterClass {
    class InnerClass {
        public void display() {
            System.out.println("Inside InnerClass");
        }
    }

    public static class StaticNestedClass {
        public void display() {
            System.out.println("Inside StaticNestedClass");
        }
    }
}

class BaseClass {
    void baseMethod() {
    }
}

class DerivedClass extends BaseClass {
    @Override
    void baseMethod() {
        System.out.println("Derived implementation");
    }
}