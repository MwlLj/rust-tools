set(git_librarys
	
)

INCLUDE_DIRECTORIES(
	"${CMAKE_CURRENT_SOURCE_DIR}/../lib_config/test/include/0.1.10/"
"${CMAKE_CURRENT_SOURCE_DIR}/../lib_config/http_server/include/0.1.0/"
"${CMAKE_CURRENT_SOURCE_DIR}/../lib_config/openssl/include/1.0.0/"

)

IF (${OS} STREQUAL "WINDOWS")
	If (${FILE_PREFIX} STREQUAL "64")
		LINK_DIRECTORIES (
			"${CMAKE_CURRENT_SOURCE_DIR}/../lib_config/test/lib/0.1.10/win64/"
"${CMAKE_CURRENT_SOURCE_DIR}/../lib_config/http_server/lib/0.1.0/win64/"
"${CMAKE_CURRENT_SOURCE_DIR}/../lib_config/openssl/lib/1.0.0/win64/"

		)
	ELSE()
		LINK_DIRECTORIES (
			"${CMAKE_CURRENT_SOURCE_DIR}/../lib_config/test/lib/0.1.10/win32/"
"${CMAKE_CURRENT_SOURCE_DIR}/../lib_config/http_server/lib/0.1.0/win32/"
"${CMAKE_CURRENT_SOURCE_DIR}/../lib_config/openssl/lib/1.0.0/win32/"

		)
	ENDIF()
ELSEIF (${OS} STREQUAL "LINUX")
	LINK_DIRECTORIES (
		"${CMAKE_CURRENT_SOURCE_DIR}/../lib_config/test/lib/0.1.10/gnu64/"
"${CMAKE_CURRENT_SOURCE_DIR}/../lib_config/http_server/lib/0.1.0/gnu64/"
"${CMAKE_CURRENT_SOURCE_DIR}/../lib_config/openssl/lib/1.0.0/gnu64/"

	)
ENDIF()

IF (${OS} STREQUAL "WINDOWS")
	If (${FILE_PREFIX} STREQUAL "64")
		IF (NOT DEFINED CMAKE_DEBUG_POSTFIX)
			SET(TARGET_NAME )
		ELSE()
		ENDIF()
	ELSE()
	ENDIF()
ELSEIF (${OS} STREQUAL "LINUX")
ENDIF()


SET(TARGET_INSTALL_DIR "${INSTALL_BIN_DIR}/service/iotas_acd_hk")

AUX_SOURCE_DIRECTORY (source/ SOURCE_LIST)

FILE(GLOB HEADER_LIST "include/*.h")

SET(LIBRARIES_NAME
	iotas_acd_client
	iotas_acd_client_db
	dahapi_iac
	dahapi
	mq_sender
	iotas_baseconfig
	aw_client
	configtool
	curlclient
	charsetconv
	sql
	log
	ospathtool
	logictool
	stringtool
	timetool
)

INCLUDE (${CMAKE_PREFIX_PATH}/09compile/GenerateLinkLibraries.cmake)

ADD_EXECUTABLE(${TARGET_NAME} ${SOURCE_LIST} ${HEADER_LIST})

SET_TARGET_PROPERTIES(${TARGET_NAME} PROPERTIES DEBUG_POSTFIX "_d")

IF (${OS} STREQUAL "WINDOWS")
		TARGET_LINK_LIBRARIES(${TARGET_NAME}
			debug test.0.1.10.64_d optimized test.0.1.10.64
debug abc-0.1.0-abcd.m.1_d optimized abc-0.1.0-abcd.m.1
debug def.0.1.0-0.1.0-abcd.m.1_d optimized def.0.1.0-0.1.0-abcd.m.1
debug openssl-1.0.0-abcd.m.1_d optimized openssl-1.0.0-abcd.m.1

		)
		
		INSTALL (TARGETS ${TARGET_NAME} RUNTIME CONFIGURATIONS Release DESTINATION ${TARGET_INSTALL_DIR})
		
		INSTALL (DIRECTORY "${INSTALL_BIN_DIR}/third_dll/hk/net/" DESTINATION ${TARGET_INSTALL_DIR})
		
		INSTALL (DIRECTORY "${INSTALL_BIN_DIR}/third_dll/openssl/" DESTINATION ${TARGET_INSTALL_DIR})
		INSTALL (DIRECTORY "${INSTALL_BIN_DIR}/third_dll/pthread/" DESTINATION ${TARGET_INSTALL_DIR})
		
		INSTALL (FILES "${INSTALL_BIN_DIR}/third_dll/anywhere_net/anywhere_net64.dll" DESTINATION ${TARGET_INSTALL_DIR})
		INSTALL (FILES "${INSTALL_BIN_DIR}/third_dll/curl/libcurl.dll" DESTINATION ${TARGET_INSTALL_DIR})
		INSTALL (FILES "${INSTALL_BIN_DIR}/third_dll/mosquitto/mosquitto.dll" DESTINATION ${TARGET_INSTALL_DIR})
		INSTALL (FILES "${INSTALL_BIN_DIR}/third_dll/mosquitto_broker/broker.dll" DESTINATION ${TARGET_INSTALL_DIR})
ELSEIF (${OS} STREQUAL "LINUX")
		TARGET_LINK_LIBRARIES(${TARGET_NAME}
						test.0.1.10.64-0.1.10
debug a123-0.1.0-abcd.m.1_d optimized a123-0.1.0-abcd.m.1
debug b456-0.1.0-abcd.m.1_d optimized b456-0.1.0-abcd.m.1
debug openssl-1.0.0-abcd.m.1_d optimized openssl-1.0.0-abcd.m.1

		)

		IF (${CMAKE_BUILD_TYPE} STREQUAL "Release")
				ADD_CUSTOM_COMMAND(TARGET ${TARGET_NAME} POST_BUILD	COMMAND echo "strip"
							COMMAND ${STRIP} ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/${TARGET_NAME}
							COMMAND mkdir -p ~/nfs/iotas/service/iotas_acd_hk
							COMMAND cp -f ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/${TARGET_NAME} ~/nfs/iotas/service/iotas_acd_hk
							COMMAND cp -Rf ${INSTALL_BIN_DIR}/third_dll/hk/net/* ~/nfs/iotas/service/iotas_acd_hk
							COMMAND mkdir -p ${TARGET_INSTALL_DIR}
							COMMAND cp -f ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/${TARGET_NAME} ${TARGET_INSTALL_DIR}
							COMMAND cp -Rf ${INSTALL_BIN_DIR}/third_dll/hk/net/* ${TARGET_INSTALL_DIR}
			)
		ENDIF()
ENDIF()


	hello

	world


