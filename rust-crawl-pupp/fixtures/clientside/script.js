document.addEventListener('DOMContentLoaded', () => {
    // Function to create and append the new node
    function createDelayedNode() {
        // Create a new div element
        const newNode = document.createElement('div');

        // Add some content to the new node
        newNode.textContent = 'This is a new node added after a delay.';

        // Add some styles to the new node
        newNode.style.padding = '10px';
        newNode.style.marginTop = '10px';
        newNode.style.backgroundColor = '#f0f0f0';
        newNode.style.border = '1px solid #ccc';
        newNode.id = 'come-find-me';

        // Append the new node to the container
        const container = document.getElementById('container');
        container.appendChild(newNode);
    }

    // Set a delay (in milliseconds)
    const delay = 30000; // 3000ms = 3 seconds

    // Use setTimeout to create and append the node after the delay
    setTimeout(createDelayedNode, delay);
});

