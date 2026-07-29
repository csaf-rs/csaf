<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
    <xsl:output method="text" />
    <xsl:template match="/">
        <!-- filename extract: version + underscore + date -->
        <xsl:value-of select="//*[local-name()='Weakness_Catalog']/@Version|//*[local-name()='Weakness_Catalog']/@Catalog_Version" /><xsl:text>_</xsl:text><xsl:value-of select="//*[local-name()='Weakness_Catalog']/@Date|//*[local-name()='Weakness_Catalog']/@Catalog_Date" /><xsl:text>&#10;</xsl:text>
        <!-- cwe entry extract -->
        <xsl:for-each select="//*[local-name()='Weakness']">
            <!-- weakness id -->
            <xsl:value-of select="@ID" />
            <xsl:text>&#09;</xsl:text>
            <!-- weakness status -->
            <xsl:value-of select="@Status" />
            <xsl:text>&#09;</xsl:text>
            <!-- weakness name -->
            <xsl:value-of select="@Name" />
            <xsl:text>&#09;</xsl:text>
            <!-- does weakness have Mapping_Notes/Usage field (bool), should be 0 before / 1 after version 4.12 -->
            <xsl:choose>
                <xsl:when test="./*[local-name()='Mapping_Notes']/*[local-name()='Usage']/text()">1</xsl:when>
                <xsl:otherwise>0</xsl:otherwise>
            </xsl:choose>
            <xsl:text>&#09;</xsl:text>
            <!-- weakness usage (empty if unavailable) -->
            <xsl:value-of select="./*[local-name()='Mapping_Notes']/*[local-name()='Usage']" />
            <xsl:text>&#10;</xsl:text>
        </xsl:for-each> 
    </xsl:template>
</xsl:stylesheet>
