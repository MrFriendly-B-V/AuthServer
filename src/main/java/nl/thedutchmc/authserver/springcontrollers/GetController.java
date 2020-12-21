package nl.thedutchmc.authserver.springcontrollers;

import java.math.BigInteger;
import java.security.SecureRandom;
import java.util.HashMap;

import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;

import nl.thedutchmc.authserver.App;
import nl.thedutchmc.authserver.Config;
import nl.thedutchmc.authserver.Http;
import nl.thedutchmc.authserver.TokenManager;

@Controller
@RequestMapping("/oauth")
public class GetController {
	
	
	// /oauth/login
	@GetMapping("login")
	public String indexPage(Model model, @RequestParam String returnUri) {
		String nonce = new BigInteger(130, new SecureRandom()).toString(16);
		String csrfToken = new BigInteger(130, new SecureRandom()).toString(32);

		HashMap<String, String> params = new HashMap<>();
		params.put("client_id", Config.clientId);
		params.put("response_type", Config.RESPONSE_TYPE);
		params.put("scope", String.join(" ", Config.SCOPES));
		params.put("redirect_uri", Config.SCHEME + Config.host + Config.REDIRECT_URI_RESPONSE);
		params.put("state", csrfToken);
		params.put("nonce", nonce);
		params.put("access_type", "offline");
		
		App.csrfTokens.add(csrfToken);
		App.returnUri.put(csrfToken, returnUri);
		
		String uriParams = Http.hashMapToString(params);
		String finalUrl = Config.GOOGLE_AUTH_URL + "?" + uriParams;
		
		model.addAttribute("targetUri", finalUrl);
		
		return "login";
	}
	
	// /oauth/grant/
	@GetMapping("grant")
	public String oauthResponse(@RequestParam String state, @RequestParam String code, Model model) {

		if(!App.csrfTokens.contains(state)) {
			return "401::Invalid state parameter";
		} else {
			App.csrfTokens.remove(state);
			
			String sessionId = new BigInteger(130, new SecureRandom()).toString(16);
			String returnUri = App.returnUri.get(state) + "?session=" + sessionId;
			new TokenManager().getToken(code, sessionId);
			
			model.addAttribute("targetUri", returnUri);
			return "grant";
		}
	}
}
