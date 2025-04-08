document.getElementById('uploadForm').addEventListener('submit', async (e) => {
    e.preventDefault();
    const imageUrl = document.getElementById('imageUrl').value;

    try {
        const response = await fetch('/api/toanime', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ imageUrl }),
        });

        if (!response.ok) throw new Error('Failed to convert image');

        const data = await response.json();
        const resultDiv = document.getElementById('result');
        resultDiv.innerHTML = `<img src="${data.resultImageUrl}" alt="Converted Anime Image" />`;
    } catch (error) {
        alert(error.message);
    }
});
