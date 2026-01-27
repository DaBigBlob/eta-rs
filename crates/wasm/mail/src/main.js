import { eta_runner } from "./worker-wasm.js";
import { EmailMessage } from "cloudflare:email";

function gimme_plaintext(raw) {
    // split headers and body at first blank line
    const match = raw.match(/^([\s\S]*?)\r?\n\r?\n([\s\S]*)$/);
    if (!match) return "";

    // unfold headers
    const headers = match[1].replace(/\r?\n[ \t]+/g, " ");
    let body = match[2];

    // accept if Content-Type is missing (defaults to text/plain)
    // or explicitly text/plain
    if (
        /content-type:/i.test(headers) &&
        !/content-type:\s*text\/plain\b/i.test(headers)
    ) {
        return "";
    }

    // quoted-printable utf8 decode
    if (/content-transfer-encoding:\s*quoted-printable/i.test(headers)) {
        body = body.replace(/=\r?\n/g, ""); // soft line breaks
        const bytes = [];
        for (let i = 0; i < body.length; i++) {
            if (body[i] === "=" && /[0-9A-Fa-f]{2}/.test(body.slice(i + 1, i + 3))) {
                bytes.push(parseInt(body.slice(i + 1, i + 3), 16));
                i += 2;
            } else {
                bytes.push(body.charCodeAt(i)); // ASCII-safe
            }
        }
        body = new TextDecoder("utf-8").decode(new Uint8Array(bytes));
    }

    return body.trim();
}



export default {
    async email(message, env, ctx) {
        const raw = await new Response(message.raw).text();

        const damail = [
            `From: "Eta Calculus Evaluator" <${message.to}>`,
            `To: ${message.from}`,
            `Subject: Eta Calculus Evaluation`,
            `Message-ID: <${crypto.randomUUID()}@MailEtaEvaler>`,
            `In-Reply-To: ${message.headers.get("Message-ID")}`,
            `References: ${message.headers.get("Message-ID")}`,
            `MIME-Version: 1.0`,
            `Content-Type: text/plain; charset=utf-8`,
            `Content-Transfer-Encoding: 8bit`,
            ``,
            `${eta_runner(gimme_plaintext(raw))}`
        ].join("\r\n");

        const replyMessage = new EmailMessage(
            message.to,
            message.from,
            damail
        );

        await message.reply(replyMessage);
    }
};
