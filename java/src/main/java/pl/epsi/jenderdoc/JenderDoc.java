package pl.epsi.jenderdoc;

import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;

public class JenderDoc {

    public static void init() {
        String os = System.getProperty("os.name").toLowerCase();
        String arch = System.getProperty("os.arch").toLowerCase();
        if (!arch.equals("amd64") && !arch.equals("x86_64"))  {
            return;
        }

        String filename, rdc;
        if (os.contains("linux")) {
            filename = "libjenderdoc.so";
            rdc = "/usr/lib/librenderdoc.so";
        } else if (os.contains("win")) {
            filename = "jenderdoc.dll";
            rdc = "C:\\Program Files\\RenderDoc\\renderdoc.dll";
        } else {
            return;
        }

        try (InputStream stream = JenderDoc.class.getResourceAsStream("/" + filename)) {
            if (stream == null) return;

            String suffix = filename.substring(filename.lastIndexOf('.'));
            Path temp = Files.createTempFile("jender-doc", suffix);

            Files.copy(stream, temp, java.nio.file.StandardCopyOption.REPLACE_EXISTING);

            temp.toFile().deleteOnExit();

            System.load(temp.toAbsolutePath().toString());
            System.load(rdc);
        } catch (Throwable ignore) {}

        RenderDoc.rd = createRenderDoc();
    }

    public static native long createRenderDoc();
    public static native void freeRenderDoc(long rd);

    public static native void startCapture(long rd);
    public static native void endCapture(long rd);

    public static native void captureNext(long rd);
    public static native void captureFrames(long rd, int count);

    public static native void captureOption(long rd, int option, int value);
    public static void captureOption(long rd, CaptureOption option, int value) {
        captureOption(rd, option.ordinal(), value);
    }

}
