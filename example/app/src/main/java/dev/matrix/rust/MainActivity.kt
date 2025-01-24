package dev.matrix.rust

import android.graphics.Bitmap
import android.os.Bundle
import android.os.Environment
import android.os.Trace
import android.util.Log
import android.view.View
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {
    private external fun callRustCode(): String
    private external fun equalTwoNum(a : Int, b: Int): Boolean
    private external fun addFile(filePath: String): Boolean
//    private external fun getBitmapInfo(bitmap: Bitmap): String

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        System.loadLibrary("rust_library")

        findViewById<View>(R.id.add).setOnClickListener {
            val internalFileName = this@MainActivity.cacheDir.absolutePath + "/test.txt"
            val externalFileName = this@MainActivity.externalCacheDir?.path + "/test.txt"
            val externalFileName2 = Environment.getExternalStoragePublicDirectory(Environment.DIRECTORY_DOWNLOADS).path + "/test.txt"

            Toast.makeText(this, "call result = ${callRustCode()}," +
                    " 1 + 2 = ${equalTwoNum(2, 2)};",
                Toast.LENGTH_SHORT).show()


            val bitmap = Bitmap.createBitmap(100, 100, Bitmap.Config.ARGB_8888);
            Log.d("Rust",   " 内部文件 = ${addFile(internalFileName)}")
            Log.d("Rust",   " 外部文件 = ${addFile(externalFileName)}")
            Log.d("Rust",   " 外部文件2 = ${addFile(externalFileName2)}")
//            Log.d("Rust",   " getBitmapInfo = ${getBitmapInfo(bitmap)}")
        }
    }
}

