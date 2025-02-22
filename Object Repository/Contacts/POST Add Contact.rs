<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Add Contact</name>
   <tag></tag>
   <elementGuidId>99381cfb-4bf6-47ac-903d-18bbf4a76a39</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;firstName\&quot;: \&quot;John\&quot;,\n    \&quot;lastName\&quot;: \&quot;Doe\&quot;,\n    \&quot;birthdate\&quot;: \&quot;1970-01-01\&quot;,\n    \&quot;email\&quot;: \&quot;jdoe@fake.com\&quot;,\n    \&quot;phone\&quot;: \&quot;8005555555\&quot;,\n    \&quot;street1\&quot;: \&quot;1 Main St.\&quot;,\n    \&quot;street2\&quot;: \&quot;Apartment A\&quot;,\n    \&quot;city\&quot;: \&quot;Anytown\&quot;,\n    \&quot;stateProvince\&quot;: \&quot;KS\&quot;,\n    \&quot;postalCode\&quot;: \&quot;12345\&quot;,\n    \&quot;country\&quot;: \&quot;USA\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.auth_token}</value>
      <webElementGuid>714c0157-6312-4eb5-b90a-095418bc43ed</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type </name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>abf420ca-aeb6-453c-96da-4ddd24120960</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://thinking-tester-contact-list.herokuapp.com/contacts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
