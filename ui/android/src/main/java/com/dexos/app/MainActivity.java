package com.dexos.app;

import android.os.Bundle;
import android.view.View;
import android.widget.Button;
import android.widget.TextView;
import androidx.appcompat.app.AppCompatActivity;

public class MainActivity extends AppCompatActivity {
    private TextView statusText;
    private Button startButton;
    private boolean kernelRunning = false;
    
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        
        statusText = findViewById(R.id.status_text);
        startButton = findViewById(R.id.start_kernel_button);
        
        startButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                toggleKernel();
            }
        });
        
        // Initialize DEX-OS kernel
        initializeDexOS();
    }
    
    private void initializeDexOS() {
        // Initialize the Rust-based DEX-OS kernel
        String result = DexOSKernel.initialize("default_config");
        statusText.setText("Kernel initialized: " + result);
    }
    
    private void toggleKernel() {
        if (kernelRunning) {
            // Stop kernel logic would go here
            kernelRunning = false;
            startButton.setText(R.string.start_kernel);
            statusText.setText(R.string.kernel_stopped);
        } else {
            // Start kernel
            kernelRunning = DexOSKernel.startKernel();
            if (kernelRunning) {
                startButton.setText(R.string.stop_kernel);
                statusText.setText(R.string.kernel_running);
            } else {
                statusText.setText("Failed to start kernel");
            }
        }
    }
}