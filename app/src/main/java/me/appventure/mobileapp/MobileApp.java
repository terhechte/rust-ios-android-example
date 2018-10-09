package me.appventure.mobileapp;

import android.app.Application;
import android.util.Log;

/**
 * Created by evgeniy on 16.03.17.
 */

public final class MobileApp extends Application {
    private static MobileApp sSelf;
    private Worker mWorker;
    private static final String TAG = "exm MobileApp";

    public MobileApp() {
        super();
        sSelf = this;
    }

    @Override
    public void onCreate() {
        Log.i(TAG, "onCreate");
        super.onCreate();
        try {
            System.loadLibrary("mobcore");
        } catch (UnsatisfiedLinkError e) {
            Log.e(TAG, "Load libary ERROR: " + e);
            return;
        }
        mWorker = new Worker();
    }

    public static MobileApp get() {
        return sSelf;
    }

    public Worker getWorker() {
        return mWorker;
    }
}
