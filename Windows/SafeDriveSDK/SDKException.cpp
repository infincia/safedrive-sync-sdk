#include "stdafx.h"
#include "SafeDriveSDK.h"
#include "sddk.h"

SDKException::SDKException(SDDKError* error) : runtime_error("SDKException"), error(error) {};

SDKErrorType SDKException::type() {
	return SDKErrorType(error->error_type);
}

std::string SDKException::message() {
	return error->message;
}

int SDKException::code() {
	return error->error_type;
}

SDKException::~SDKException() {
	sddk_free_error(&error);
} 