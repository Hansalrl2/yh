import express from 'express';
import axios from 'axios';
import puppeteer from 'puppeteer-core';
import { chromium } from '@sparticuz/chromium';

const app = express();
app.use(express.json());

app.post('/api/toanime', async (req, res) => {
    const { imageUrl } = req.body;

    if (!imageUrl) {
        return res.status(400).send('Image URL is required.');
    }

    try {
        // Download image
        const tempImagePath = `/tmp/image-${Date.now()}.jpg`;
        const response = await axios({
            url: imageUrl,
            responseType: 'arraybuffer',
        });
        require('fs').writeFileSync(tempImagePath, Buffer.from(response.data));

        // Convert image using Puppeteer
        const browser = await puppeteer.launch({
            args: chromium.args,
            defaultViewport: chromium.defaultViewport,
            executablePath: await chromium.executablePath,
            headless: true,
        });
        const page = await browser.newPage();
        await page.goto('https://taoanhdep.com/tao-anh-anime-ai-theo-anh-cua-ban/#profile');
        const inputUploadHandle = await page.$('input[type="file"]');
        await inputUploadHandle.uploadFile(tempImagePath);
        await page.waitForSelector('.result-image', { timeout: 60000 });
        const resultImageUrl = await page.$eval('.result-image img', (img) => img.src);
        await browser.close();

        res.json({ resultImageUrl });
    } catch (error) {
        console.error(error);
        res.status(500).send('Error processing image.');
    }
});

export default app;
