package dev.matrix.rust

import android.os.Bundle
import android.os.Trace
import android.util.Log
import android.view.View
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {
    private external fun callRustCode(): String

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        System.loadLibrary("rust_library")
        findViewById<View>(R.id.container).setOnClickListener {
            Toast.makeText(this, callRustCode(), Toast.LENGTH_SHORT).show()
        }
    }
}
