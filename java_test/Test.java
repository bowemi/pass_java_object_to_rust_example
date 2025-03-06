package java_test;

public class Test {
    public static native String init_rust(Test t);
    public int callback(String s) {
        System.out.println(s);

        return 0;
    }

    public Test testClone() {
        return this;
    }

    public static void setup() {
        Test t = new Test();
        init_rust(t);
    }

    public static void main(String[] args) {
        System.loadLibrary("duchess_tests");
        setup();
        try
        {
            Thread.sleep(10000);
        }
        catch(InterruptedException ex)
        {
        }
    }
}

