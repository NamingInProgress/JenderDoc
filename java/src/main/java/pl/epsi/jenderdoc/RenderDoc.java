package pl.epsi.jenderdoc;

public class RenderDoc {

    static long rd;

    public static Capture capture() {
        verifyInit();
        return new Capture();
    }

    public static void captureNextFrame() {
        verifyInit();
        JenderDoc.captureNext(rd);
    }

    public static void captureNextFrames(int frames) {
        verifyInit();
        JenderDoc.captureFrames(rd, frames);
    }

    public static void captureOption(CaptureOption option, int value) {
        verifyInit();
        JenderDoc.captureOption(rd, option.ordinal(), value);
    }

    public static void free() {
        verifyInit();
        JenderDoc.freeRenderDoc(rd);
    }

    private static void verifyInit() {
        if (rd == 0L) throw new RuntimeException("Tried to use RenderDoc before calling init! Are you sure you are in release mode?");
    }

    public static class Capture implements AutoCloseable {

        public Capture() {
            JenderDoc.startCapture(rd);
        }

        @Override
        public void close() throws Exception {
            JenderDoc.endCapture(rd);
        }

    }

}
