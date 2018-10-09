// Automaticaly generated by rust_swig
package me.appventure.mobileapp;
import android.support.annotation.NonNull;

public final class Worker {

    public Worker()  {

        mNativeObj = init();
    }
    private static native long init() ;

    public final String action(@NonNull String a0)  {

        return do_action(mNativeObj, a0);
    }
    private static native String do_action(long me, String a0) ;

    public synchronized void delete() {
        if (mNativeObj != 0) {
            do_delete(mNativeObj);
            mNativeObj = 0;
       }
    }
    @Override
    protected void finalize() throws Throwable {
        try {
            delete();
        }
        finally {
             super.finalize();
        }
    }
    private static native void do_delete(long me);
    /*package*/ long mNativeObj;
}